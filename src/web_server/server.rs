use axum::Router;
use axum::routing::get;
use tracing::info;

use crate::web_server::data::ServerData;

#[derive(Default)]
pub struct Server {
    data: ServerData,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    /// # Panics
    ///
    /// Panics if server fails to run.
    pub async fn run(self) {
        let app = Router::new().route(
            "/",
            get(async move || {
                format!(
                    "Stage set states:
{:?}

Last temperature: {} \u{00B0}F",
                    self.data.set_states.read().await,
                    self.data.last_temperature.read().await
                )
            }),
        );
        let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

        info!("Starting web server");
        axum::serve(listener, app).await.unwrap();

        // TODO: Server should exit on ServerData dropping
    }
}
