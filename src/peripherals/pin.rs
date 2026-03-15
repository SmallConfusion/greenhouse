use std::time::Duration;

use crate::peripheral::{command_preset::Peripheral, peripheral_command::PeripheralCommand};
use derive_more::Display;
use rppal::gpio::{Gpio, OutputPin};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::{sync::watch::Receiver, task::JoinHandle};
use tracing::{debug, error, trace};

#[derive(Debug, Deserialize, JsonSchema, Clone, Copy, Display, PartialEq, Eq)]
pub enum PinState {
    Off,
    On,
}

impl PeripheralCommand for PinState {}

#[derive(Debug)]
pub struct Pin {
    pin: Option<OutputPin>,
    index: u8,
}

impl Pin {
    pub fn new(index: u8) -> Self {
        let pin = (|| {
            Gpio::new()
                .inspect_err(|e| error!("Cannot get gpio: {e}"))
                .ok()?
                .get(index)
                .inspect_err(|e| error!("Cannot get pin {index}: {e}"))
                .ok()
                .map(rppal::gpio::Pin::into_output)
        })();

        if pin.is_some() {
            debug!("Pin {index} initialized successfully");
        } else {
            debug!("Pin {index} failed initialization");
        }

        Self { pin, index }
    }

    pub fn set(&mut self, state: &PinState) {
        if let Some(pin) = self.pin.as_mut() {
            match state {
                PinState::Off => pin.set_high(),
                PinState::On => pin.set_low(),
            }
        } else {
            error!("Pin {} is not initialized", self.index)
        }

        trace!("Pin {} set {}", self.index, state);
    }
}

impl Peripheral for Pin {
    type Command = PinState;

    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                error!("Not implemented");
                tokio::time::sleep(Duration::from_hours(1)).await
            }
        })
    }
}
