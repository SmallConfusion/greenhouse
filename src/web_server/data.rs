use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;
use tracing::{error, trace};

#[derive(Clone, Debug)]
pub enum Info {
    /// (Set name, Stage name).
    StateEnter(String, String),
}

impl Display for Info {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StateEnter(set, stage) => {
                write!(f, "Stage Entered {{ Set: \"{set}\" Stage: \"{stage}\" }}")
            }
        }
    }
}

#[derive(Clone)]
pub struct InfoChannel {
    sender: UnboundedSender<Info>,
}

impl InfoChannel {
    pub fn new() -> (Self, UnboundedReceiver<Info>) {
        let (sender, receiver) = unbounded_channel();

        (Self { sender }, receiver)
    }

    pub fn send_info(&mut self, info: Info) {
        trace!("Sending info {info}");

        match self.sender.send(info) {
            Ok(()) => (),
            Err(err) => error!("Error sending info: {err}"),
        }
    }
}

#[derive(Clone, Default)]
pub(super) struct ServerData {
    pub set_states: Arc<RwLock<HashMap<String, String>>>,
}

impl ServerData {
    pub fn run(&self) -> (InfoChannel, JoinHandle<()>) {
        let (info_channel, mut receiver) = InfoChannel::new();

        let our_self = self.clone();

        let join = tokio::spawn(async move {
            loop {
                let info_opt = receiver.recv().await;

                let Some(info) = info_opt else {
                    error!("Server info thread closed");
                    break;
                };

                trace!("Got info {info}");

                match info {
                    Info::StateEnter(set, stage) => {
                        our_self.set_states.write().await.insert(set, stage);
                    }
                }
            }
        });

        (info_channel, join)
    }
}
