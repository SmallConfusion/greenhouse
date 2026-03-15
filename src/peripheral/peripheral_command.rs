use std::fmt::Debug;
use tokio::{sync::watch::Receiver, task::JoinHandle};

pub trait PeripheralCommand: Debug + Clone + Send + 'static {}

pub trait Peripheral: Debug + 'static {
    type Command: PeripheralCommand;
    fn run_loop(self, receiver: Receiver<Self::Command>) -> JoinHandle<()>;
}
