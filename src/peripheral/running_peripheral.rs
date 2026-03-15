use crate::peripheral::{
    command_preset::{CommandPreset, Peripheral},
    peripheral_command::PeripheralCommand,
};
use tokio::{
    sync::watch::{Sender, channel},
    task::JoinHandle,
};

pub struct RunningPeripheral<T: PeripheralCommand> {
    pub sender: Sender<T>,
    pub join: JoinHandle<()>,
}

impl<T: PeripheralCommand> RunningPeripheral<T> {
    pub fn create_from_peripheral(peripheral: impl Peripheral<Command = T>, default: T) -> Self {
        let (sender, receiver) = channel(default);
        let join = peripheral.run_loop(receiver);

        Self { sender, join }
    }

    pub fn make_command_preset(&mut self, command: T) -> CommandPreset<T> {
        CommandPreset::new(self.sender.clone(), command)
    }
}
