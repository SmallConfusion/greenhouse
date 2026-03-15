use std::time::Duration;

use crate::controller::stage::Stage;
use derive_more::Constructor;
use tracing::trace;
pub mod stage;

pub trait Condition {
    // TODO: Move to module
    fn is_met(&self) -> bool;
}

#[derive(Debug, Constructor)]
pub struct Controller {
    stage_sets: Vec<StageSet>,
}

impl Controller {
    pub async fn run(mut self) {
        for stage_set in self.stage_sets.iter_mut() {
            for stage in stage_set.set.iter_mut() {
                // TODO: Replace with an actual run
                stage.enter();
            }
        }

        tokio::time::sleep(Duration::from_secs(2)).await;

        trace!("Done running");
    }
}

#[derive(Debug, Constructor)]
pub struct StageSet {
    set: Vec<Stage>,
}
