use crate::peripheral::peripheral_command::AnyCommand;
use crate::peripheral::{
    command_preset::{CommandPreset, GenericCommand},
    Peripheral,
};
use std::any::type_name;
use tokio::{
    sync::watch::{channel, Sender},
    task::JoinHandle,
};
use tracing::error;

pub struct RunningPeripheral<T: Peripheral> {
    sender: Sender<T::Command>,
    default: T::Command,
}

impl<T: Peripheral> RunningPeripheral<T> {
    pub fn create_from_peripheral(peripheral: T, default: T::Command) -> (Self, JoinHandle<()>) {
        let (sender, mut receiver) = channel(default.clone());
        receiver.mark_changed();
        let join = peripheral.run_loop(receiver);

        (Self { sender, default }, join)
    }
}

pub trait GenericPeripheral {
    fn create_command(&self, command_opt: Option<AnyCommand>) -> Box<dyn GenericCommand>;
}

impl<T: Peripheral> GenericPeripheral for RunningPeripheral<T> {
    fn create_command(&self, command_opt: Option<AnyCommand>) -> Box<dyn GenericCommand> {
        let new_value = command_opt
            .and_then(|command_any| {
                command_any
                    .into_any()
                    .downcast::<T::Command>()
                    .inspect_err(|_| {
                        error!(
                            "Mismatched types! {} needs a command of type {}.",
                            type_name::<T>(),
                            type_name::<T::Command>()
                        );
                    })
                    .map(|command| *command)
                    .ok()
            })
            .unwrap_or_else(|| self.default.clone());

        let preset = CommandPreset::new(self.sender.clone(), new_value);
        Box::new(preset)
    }
}
