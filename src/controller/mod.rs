pub mod stage;
pub mod stage_set;

use crate::controller::stage_set::StageSet;
use derive_more::Constructor;
use futures::{StreamExt, stream::FuturesUnordered};
use tokio::task::JoinHandle;
use tracing::error;

#[derive(Debug, Constructor)]
pub struct Controller {
    stage_sets: Vec<StageSet>,
    join_handles: Vec<JoinHandle<()>>,
}

impl Controller {
    pub async fn run(mut self) {
        for set in self.stage_sets {
            self.join_handles.push(set.run());
        }

        let mut futures = FuturesUnordered::new();

        for h in self.join_handles {
            futures.push(h);
        }

        futures.next().await;

        error!(
            "One of our async tasks has ended. This could be because a defined pin was not used or it could be because of a horrible error"
        );
    }
}
