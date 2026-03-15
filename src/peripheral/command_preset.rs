use crate::peripheral::peripheral_command::PeripheralCommand;
use derive_more::Constructor;
use std::fmt::Debug;
use tokio::sync::watch::Sender;
use tracing::error;

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

pub trait GenericCommand: Debug + Send {
    fn send(&mut self);
}

impl<T: PeripheralCommand> GenericCommand for CommandPreset<T> {
    fn send(&mut self) {
        self.send();
    }
}
