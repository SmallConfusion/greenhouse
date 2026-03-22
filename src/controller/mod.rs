mod stage;
mod stage_set;

use derive_more::Constructor;
use futures::StreamExt as _;
use futures::stream::FuturesUnordered;
pub use stage::*;
pub use stage_set::*;
use tokio::task::JoinHandle;
use tracing::error;

use crate::web_server::InfoChannel;

#[derive(Debug, Constructor)]
pub struct Controller {
    stage_sets: Vec<StageSet>,
    join_handles: Vec<JoinHandle<()>>,
}

impl Controller {
    pub async fn run(mut self, info_channel: InfoChannel) {
        for set in self.stage_sets {
            self.join_handles.push(set.run(info_channel.clone()));
        }

        let mut futures = FuturesUnordered::new();

        for handle in self.join_handles {
            futures.push(handle);
        }

        futures.next().await;

        error!(
            "One of our async tasks has ended. This could be because a defined pin was not used or it could be because of a horrible error"
        );
    }
}
