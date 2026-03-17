use std::time::Duration;

use crate::controller::stage::Stage;
use tokio::task::JoinHandle;

#[derive(Debug)]
pub struct StageSet {
    stage: Vec<Stage>,
    default: Stage,
    is_in_default: bool,
}

impl StageSet {
    pub const fn new(stage: Vec<Stage>, default: Stage) -> Self {
        Self {
            stage,
            default,
            is_in_default: true,
        }
    }

    pub fn run(mut self) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let entry_stage_opt = self.stage.iter_mut().find(|stage| stage.can_enter());

                if let Some(entry_stage) = entry_stage_opt {
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
