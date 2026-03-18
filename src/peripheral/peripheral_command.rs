use std::any::Any;
use std::fmt::{Debug, Display};

pub trait PeripheralCommand: Debug + Display + Clone + Send + Sync + 'static {}

pub struct AnyCommand {
    command: Box<dyn Any>,
}

impl AnyCommand {
    pub fn new<T: PeripheralCommand>(command: T) -> Self {
        Self {
            command: Box::new(command),
        }
    }

    pub fn into_any(self) -> Box<dyn Any> {
        self.command
    }
}
