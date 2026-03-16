use std::time::Duration;

use crate::peripheral::{
    Peripheral,
    implementation::pin::{Pin, PinState},
    peripheral_command::PeripheralCommand,
};
use derive_more::{Deref, From};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::{select, time::Instant};
use tracing::{error, trace};

#[derive(Debug)]
pub struct Vent {
    on: Pin,
    off: Pin,

    current_state: VentState,
}

#[derive(Debug, Deserialize, JsonSchema, From, Deref, Clone, Copy)]
pub struct VentState(pub f32);

impl PeripheralCommand for VentState {}

impl Peripheral for Vent {
    type Command = VentState;

    fn run_loop(
        mut self,
        mut receiver: tokio::sync::watch::Receiver<Self::Command>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(err) = receiver.changed().await {
                    error!(
                        "Vent {:?} {:?}'s async loop receiver returned an error: {err}", // TODO: Improve error message
                        self.on, self.off,
                    );
                    return;
                }

                let new_state = *receiver.borrow();
                trace!("Vent {self:?} received {new_state:?}");

                let diff = *new_state - *self.current_state;
                let dist = diff.abs();

                let pin = if diff > 0.0 {
                    &mut self.on
                } else {
                    &mut self.off
                };

                let start_time = Instant::now();
                pin.set(&PinState::On);

                select! {
                    () = tokio::time::sleep(Duration::from_secs_f32(dist)) => (),
                    _ = receiver.changed() => receiver.mark_changed(),
                };

                let end_time = Instant::now();
                pin.set(&PinState::Off);

                let moved_time = end_time - start_time;
                self.current_state.0 += moved_time.as_secs_f32();
            }
        })
    }
}

impl Vent {
    pub fn new(on: Pin, off: Pin) -> Self {
        Self {
            on,
            off,
            current_state: 0.0.into(),
        }
    }
}
