use crate::controller::stage::Stage;
use derive_more::Constructor;

pub trait Condition {
    fn is_met(&self) -> bool;
}

#[derive(Debug, Constructor)]
pub struct Controller {
    stage_sets: Vec<StageSet>,
}

#[derive(Debug, Constructor)]
pub struct StageSet {
    set: Vec<Stage>,
}

pub mod stage {
    use std::fmt::Debug;

    use crate::peripheral::{command_preset::CommandPreset, peripheral_command::PeripheralCommand};

    trait GenericCommand: Debug {
        fn send(&mut self);
    }

    impl<T: PeripheralCommand> GenericCommand for CommandPreset<T> {
        fn send(&mut self) {
            self.send();
        }
    }

    #[derive(Debug)]
    pub struct Stage {
        entry: Vec<Box<dyn GenericCommand>>,
        // condition: Box<dyn Condition>,
    }

    impl Default for Stage {
        fn default() -> Self {
            Self::new()
        }
    }

    impl Stage {
        #[must_use] 
        pub fn new(//condition: Box<dyn Condition>
        ) -> Self {
            Self {
                // condition,
                entry: Vec::new(),
            }
        }

        pub fn add_command<T: PeripheralCommand + 'static>(&mut self, command: CommandPreset<T>) {
            self.entry.push(Box::new(command));
        }
    }
}
