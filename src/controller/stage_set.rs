use std::time::Duration;

use tokio::task::JoinHandle;

use crate::controller::stage::Stage;
use crate::web_server::data::{Info, InfoChannel};

#[derive(Debug)]
pub struct StageSet {
    stage: Vec<Stage>,
    default: Stage,
    is_in_default: bool,
    name: String,
}

impl StageSet {
    pub const fn new(stage: Vec<Stage>, default: Stage, name: String) -> Self {
        Self {
            stage,
            default,
            is_in_default: true,
            name,
        }
    }

    pub fn run(mut self, mut info_handle: InfoChannel) -> JoinHandle<()> {
        tokio::spawn(async move {
            loop {
                let entry_stage_opt = self.stage.iter_mut().find(|stage| stage.can_enter());

                if let Some(entry_stage) = entry_stage_opt {
                    self.is_in_default = false;

                    entry_stage.enter();

                    info_handle.send_info(Info::StateEnter(
                        self.name.clone(),
                        entry_stage.name().to_owned(),
                    ));

                    while !entry_stage.should_exit() {
                        tokio::time::sleep(Duration::new(1, 0)).await;
                    }
                } else {
                    if !self.is_in_default {
                        self.default.enter();

                        info_handle.send_info(Info::StateEnter(
                            self.name.clone(),
                            self.default.name().to_owned(),
                        ));

                        self.is_in_default = true;
                    }

                    tokio::time::sleep(Duration::new(1, 0)).await;
                }
            }
        })
    }
}
