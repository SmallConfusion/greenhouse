mod command_preset;
pub mod implementation;
mod peripheral_command;
mod running_peripheral;

use std::fmt::Debug;

pub use command_preset::*;
pub use peripheral_command::*;
pub use running_peripheral::*;
use tokio::sync::watch::Receiver;
use tokio::task::JoinHandle;

pub trait Peripheral: Debug + 'static {
    type Command: PeripheralCommand;
    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()>;
}
