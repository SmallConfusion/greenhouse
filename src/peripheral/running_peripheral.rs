use crate::{
    controller::stage::GenericCommand,
    peripheral::{
        command_preset::{CommandPreset, GenericPeripheral},
        peripheral_command::Peripheral,
    },
};
use std::any::Any;
use tokio::{
    sync::watch::{Sender, channel},
    task::JoinHandle,
};
use tracing::error;

pub struct RunningPeripheral<T: Peripheral> {
    sender: Sender<T::Command>,
    default: T::Command,
}

impl<T: Peripheral> RunningPeripheral<T> {
    pub fn create_from_peripheral(peripheral: T, default: T::Command) -> (Self, JoinHandle<()>) {
        let (sender, receiver) = channel(default.clone());
        let join = peripheral.run_loop(receiver);

        (Self { sender, default }, join)
    }
}

impl<T: Peripheral> GenericPeripheral for RunningPeripheral<T> {
    fn create_command(&self, command: Option<Box<dyn Any>>) -> Box<dyn GenericCommand> {
        let new_value = command
            .map(|c| {
                c.downcast::<T::Command>()
                    .inspect_err(|_e| error!("Mismatched types"))
                    .map(|c| *c)
                    .ok()
            })
            .flatten()
            .unwrap_or(self.default.clone());

        let preset = CommandPreset::new(self.sender.clone(), new_value);
        Box::new(preset)
    }
}
