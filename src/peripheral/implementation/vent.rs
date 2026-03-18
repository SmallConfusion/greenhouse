use std::time::Duration;

use crate::peripheral::{
    Peripheral,
    implementation::pin::{Pin, PinState},
    peripheral_command::PeripheralCommand,
};
use derive_more::{Deref, Display, From};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::{select, time::Instant};
use tracing::{debug, error};

#[derive(Debug)]
pub struct Vent {
    on: Pin,
    off: Pin,

    current_state: VentState,
}

#[derive(Debug, Deserialize, JsonSchema, From, Deref, Clone, Copy, Display)]
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
                    error!("{self}'s async loop receiver returned an error: {err}");
                    return;
                }

                let new_state = *receiver.borrow();
                debug!("Vent {self} received command {new_state}");

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
                }

                let moved_time = start_time.elapsed();

                pin.set(&PinState::Off);

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

impl std::fmt::Display for Vent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vent({}, {})", self.on, self.off)
    }
}
