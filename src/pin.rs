use derive_more::Display;
use rppal::gpio::{Gpio, OutputPin};
use schemars::JsonSchema;
use serde::Deserialize;
use tracing::{Level, debug, error, span, trace};

#[derive(Debug, Deserialize, JsonSchema, Clone, Copy, Display, PartialEq, Eq)]
pub enum PinState {
    Off,
    On,
}

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
                .map(|pin| pin.into_output())
        })();

        if pin.is_some() {
            debug!("Pin {index} initialized successfully");
        } else {
            debug!("Pin {index} failed initialization");
        }

        Self { pin, index }
    }

    pub fn set(&mut self, state: &PinState) {
        match self.pin.as_mut() {
            Some(pin) => match state {
                PinState::Off => pin.set_high(),
                PinState::On => pin.set_low(),
            },
            None => error!("Pin {} is not initialized", self.index),
        }

        trace!("Pin {} set {}", self.index, state);
    }
}
