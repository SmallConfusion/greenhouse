use crate::{
    peripheral::Peripheral, peripheral::peripheral_command::PeripheralCommand,
    peripherals::pin::Pin,
};
use derive_more::{Constructor, Deref, From};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::error;

#[derive(Debug, Constructor)]
pub struct Vent {
    on: Pin,
    off: Pin,
}

#[derive(Debug, Deserialize, JsonSchema, From, Deref, Clone, Copy)]
pub struct VentState(pub f32);

impl PeripheralCommand for VentState {}

impl Peripheral for Vent {
    type Command = VentState;

    fn run_loop(
        self,
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
                error!("Vent stuff not implemented but {new_state:?} was received");
            }
        })
    }
}
