pub mod command_preset;
pub mod implementation;
pub mod peripheral_command;
pub mod running_peripheral;

use crate::peripheral::peripheral_command::PeripheralCommand;
use std::fmt::Debug;
use tokio::{sync::watch::Receiver, task::JoinHandle};

pub trait Peripheral: Debug + 'static {
    type Command: PeripheralCommand;
    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()>;
}
