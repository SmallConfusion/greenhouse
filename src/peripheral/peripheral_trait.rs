use crate::{
    peripheral::peripheral_command::PeripheralCommand,
    pin::{Pin, PinState},
};
use derive_more::{Constructor, Deref, From};
use tokio::{
    sync::watch::{Receiver, Sender, channel},
    task::JoinHandle,
};
use tracing::{error, trace};

pub trait Peripheral<T: PeripheralCommand> {
    fn run_loop(self, receiver: Receiver<T>) -> JoinHandle<()>;
}

#[derive(Constructor)]
pub struct CommandPreset<T: PeripheralCommand> {
    sender: Sender<T>,
    value: T,
}

impl<T: PeripheralCommand> CommandPreset<T> {
    pub fn send(&mut self) {
        if let Err(err) = self.sender.send(self.value.clone()) {
            error!("Cannot send value {:?}: {err}", self.value);
        }
    }
}
