pub mod stage;

use crate::controller::stage::Stage;
use derive_more::Constructor;
use std::time::Duration;
use tokio::task::JoinHandle;
use tracing::trace;

#[derive(Debug, Constructor)]
pub struct Controller {
    stage_sets: Vec<StageSet>,
    join_handles: Vec<JoinHandle<()>>,
}

impl Controller {
    pub async fn run(mut self) {
        for stage_set in &mut self.stage_sets {
            for stage in &mut stage_set.set {
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
