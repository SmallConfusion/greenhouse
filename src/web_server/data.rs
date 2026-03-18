use std::collections::HashMap;
use std::sync::Arc;

use tokio::sync::RwLock;
use tokio::sync::mpsc::{UnboundedSender, unbounded_channel};
use tokio::task::JoinHandle;
use tracing::error;

pub enum Info {
    StateEnter(String, String),
    TemperatureGot(f32),
}

#[derive(Clone, Default)]
pub(super) struct ServerData {
    pub set_states: Arc<RwLock<HashMap<String, String>>>,
    pub last_temperature: Arc<RwLock<f32>>,
}

impl ServerData {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&self) -> (UnboundedSender<Info>, JoinHandle<()>) {
        let (sender, mut receiver) = unbounded_channel::<Info>();

        let our_self = self.clone();

        let join = tokio::spawn(async move {
            loop {
                let info_opt = receiver.recv().await;

                let Some(info) = info_opt else {
                    error!("Server info thread closed");
                    break;
                };

                match info {
                    Info::StateEnter(set, stage) => {
                        our_self.set_states.write().await.insert(set, stage);
                    }
                    Info::TemperatureGot(temperature) => {
                        *our_self.last_temperature.write().await = temperature;
                    }
                }
            }
        });

        (sender, join)
    }
}
