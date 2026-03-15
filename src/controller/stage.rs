use std::fmt::Debug;

use derive_more::Constructor;

use crate::peripheral::{command_preset::CommandPreset, peripheral_command::PeripheralCommand};

pub trait GenericCommand: Debug {
    fn send(&mut self);
}

impl<T: PeripheralCommand> GenericCommand for CommandPreset<T> {
    fn send(&mut self) {
        self.send();
    }
}

#[derive(Debug, Constructor)]
pub struct Stage {
    entry: Vec<Box<dyn GenericCommand>>,
    // condition: Box<dyn Condition>,
}

impl Stage {
    pub fn enter(&mut self) {
        for command in &mut self.entry {
            command.send();
        }
    }
}
