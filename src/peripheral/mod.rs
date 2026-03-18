pub mod command_preset;
pub mod implementation;
pub mod peripheral_command;
pub mod running_peripheral;

use std::fmt::Debug;

use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

use crate::peripheral::peripheral_command::PeripheralCommand;

pub trait Peripheral: Debug + 'static {
    type Command: PeripheralCommand;
    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()>;
}
