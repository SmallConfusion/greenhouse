use derive_more::Display;
use rppal::gpio::{Gpio, OutputPin};
use schemars::JsonSchema;
use serde::Deserialize;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;
use tracing::{debug, error, info};

use crate::peripheral::Peripheral;
use crate::peripheral::peripheral_command::PeripheralCommand;

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
                .inspect_err(|err| error!("Cannot get gpio: {err}"))
                .ok()?
                .get(index)
                .inspect_err(|err| error!("Cannot get pin {index}: {err}"))
                .ok()
                .map(rppal::gpio::Pin::into_output)
        })();

        if pin.is_some() {
            info!("Pin {index} initialized successfully");
        } else {
            error!("Pin {index} failed initialization");
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
            error!("{self} is not initialized");
        }

        debug!("{self} set {state}");
    }
}

impl Peripheral for Pin {
    type Command = PinState;

    fn run_loop(mut self, mut receiver: Receiver<Self::Command>) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                if let Err(err) = receiver.changed().await {
                    error!(
                        "Pin {}'s async loop receiver returned an error: {err}",
                        self.index
                    );
                    return;
                }

                let new = *receiver.borrow();
                debug!("{self} received command {}", new);
                self.set(&new);
            }
        })
    }
}

impl std::fmt::Display for Pin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Pin({})", self.index)
    }
}
