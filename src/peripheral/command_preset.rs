use crate::{
    controller::stage::GenericCommand,
    peripheral::peripheral_command::PeripheralCommand,
};
use derive_more::Constructor;
use std::{any::Any, fmt::Debug};
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

pub trait GenericPeripheral {
    fn create_command(&self, command: Option<Box<dyn Any>>) -> Box<dyn GenericCommand>;
}
