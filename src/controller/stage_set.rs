use std::time::Duration;

use crate::controller::stage::Stage;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct StageSet {
    set: Vec<Stage>,
    default: Stage,
    is_in_default: bool,
}

impl StageSet {
    pub fn new(set: Vec<Stage>, default: Stage) -> Self {
        Self {
            set,
            default,
            is_in_default: true,
        }
    }

    pub fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let entry_stage = self.set.iter_mut().find(|s| s.can_enter());

                if let Some(entry_stage) = entry_stage {
                    self.is_in_default = false;

                    entry_stage.enter();

                    while !entry_stage.should_exit() {
                        tokio::time::sleep(Duration::new(1, 0)).await;
                    }
                } else {
                    if !self.is_in_default {
                        self.default.enter();
                        self.is_in_default = true;
                    }

                    tokio::time::sleep(Duration::new(1, 0)).await;
                }
            }
        })
    }
}
