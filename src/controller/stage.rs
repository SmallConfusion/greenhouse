use std::fmt::Debug;

use derive_more::Constructor;
use never::Never;
use tracing::info;

use crate::{condition::Condition, peripheral::command_preset::GenericCommand};

impl Condition for Never {
    fn is_met(&self) -> bool {
        unreachable!()
    }
}

#[derive(Debug, Constructor)]
pub struct Stage {
    entry: Vec<Box<dyn GenericCommand>>,

    /// If this is not set, stage will enter whenever given the chance.
    condition: Option<Box<dyn Condition>>,

    /// If this condition is met, the stage will not exit.
    stay_condition: Option<Box<dyn Condition>>,

    name: Option<String>,
}

impl Condition for Option<Box<dyn Condition>> {
    fn is_met(&self) -> bool {
        self.iter().map(|c| c.is_met()).next().unwrap_or(true)
    }
}

impl Stage {
    pub fn enter(&mut self) {
        if let Some(name) = &self.name {
            info!("Entering stage {name}");
        } else {
            info!("Entering unnamed stage");
        }

        for command in &mut self.entry {
            command.send();
        }
    }

    pub fn can_enter(&self) -> bool {
        self.condition.is_met()
    }

    pub fn should_exit(&self) -> bool {
        match &self.stay_condition {
            Some(c) => !c.is_met(),
            None => !self.condition.is_met(),
        }
    }
}
