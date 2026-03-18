use std::fmt::Debug;

use derive_more::Constructor;
use never::Never;
use tracing::info;

use crate::condition::Condition;
use crate::peripheral::command_preset::GenericCommand;

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

    name: String,
}

impl Condition for Option<Box<dyn Condition>> {
    fn is_met(&self) -> bool {
        self.iter()
            .map(|condition| condition.is_met())
            .next()
            .unwrap_or(true)
    }
}

impl Stage {
    pub fn enter(&mut self) {
        info!("Entering stage {}", self.name);

        for command in &mut self.entry {
            command.send_generic();
        }
    }

    pub fn can_enter(&self) -> bool {
        self.condition.is_met()
    }

    pub fn should_exit(&self) -> bool {
        !self.stay_condition.as_ref().map_or_else(
            || self.condition.is_met(),
            |stay_condition| stay_condition.is_met(),
        )
    }
}
