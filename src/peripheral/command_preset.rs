use std::fmt::Debug;

use crate::peripheral::peripheral_command::PeripheralCommand;
use derive_more::Constructor;
use tokio::{
    sync::watch::{Receiver, Sender},
    task::JoinHandle,
};
use tracing::error;

pub trait Peripheral: Debug {
    type Command: PeripheralCommand;
    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()>;
}

#[derive(Debug, Constructor)]
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
