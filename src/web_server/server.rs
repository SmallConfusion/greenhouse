use std::fs;

use axum::Router;
use axum::routing::get;
use axum::serve::Serve;
use tokio::net::TcpListener;
use tracing::info;

use crate::input::get_temperature;
use crate::web_server::{InfoChannel, ServerData};

#[derive(Default)]
pub struct Server {
    data: ServerData,
}

impl Server {
    /// # Panics
    ///
    /// Panics if server fails to run.
    pub async fn run(self) -> (InfoChannel, Serve<TcpListener, Router, Router>) {
        let (info_channel, _handle) = self.data.run();

        let app = Router::new().route(
            "/",
            get(async move || {
                format!(
                    "Stage set states:
{:#?}

Last temperature: {} \u{00B0}F

Log:
{}
",
                    self.data.set_states.read().await,
                    get_temperature(),
                    fs::read_to_string("log.txt")
                        .unwrap_or_else(|err| format!("Error getting log: {err}"))
                )
            }),
        );
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

        info!("Starting web server");

        (info_channel, axum::serve(listener, app))
    }
}
