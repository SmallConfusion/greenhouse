use std::time::Duration;

use derive_more::{Constructor, Deref, From};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::error;

use crate::{
    peripheral::{
        command_preset::Peripheral, peripheral_command::PeripheralCommand,
        running_peripheral::RunningPeripheral,
    },
    peripherals::pin::{Pin, PinState},
};

#[derive(Debug, Constructor)]
pub struct Vent {
    on: Pin,
    off: Pin,
}

#[derive(Debug, Deserialize, JsonSchema, From, Deref, Clone)]
pub struct VentState(pub f32);

impl PeripheralCommand for VentState {}

impl Peripheral for Vent {
    type Command = VentState;

    fn run_loop(
        self,
        receiver: tokio::sync::watch::Receiver<Self::Command>,
    ) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                error!("Not implemented");
                tokio::time::sleep(Duration::from_hours(1)).await
            }
        })
    }
}
