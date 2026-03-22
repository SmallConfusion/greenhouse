pub mod condition;
pub mod config;
pub mod controller;
pub mod input;
pub mod peripheral;
pub mod web_server;

use std::fs;
use std::fs::File;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::serve::Serve;
use color_eyre::eyre::Result;
use schemars::schema_for;
use tokio::net::TcpListener;
use tokio::select;
use tracing::level_filters::LevelFilter;
use tracing::{error, trace};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{EnvFilter, Layer as _, Registry};
use web_server::InfoChannel;

use crate::config::{Config, parse_args};
use crate::input::init_temperature;
use crate::web_server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();

    let args = parse_args();

    #[allow(clippy::useless_let_if_seq, reason = "Bad suggestion")]
    let mut printed = false;

    if args.schema {
        let schema = serde_json::to_string(&schema_for!(Config))?;
        println!("{schema}");
        printed = true;
    }

    if args.template {
        let content = include_str!("config/config.yaml");
        println!("{content}");
        printed = true;
    }

    if args.compose {
        let content = include_str!("../compose.yaml");
        println!("{content}");
        printed = true;
    }

    if !args.run_controller {
        if !printed {
            println!("Run with -r to run the controller");
        }

        return Ok(());
    }

    let Config {
        controller_desc,
        temperature_path,
    } = Config::load(&args.config);

    init_temperature(temperature_path);

    let controller = controller_desc.initialize();

    let (info_channel, server_join) = start_server().await;
    let controller_join = controller.run(info_channel);

    select! {
        () = controller_join => error!("Controller loop ended"),
        _ = server_join => error!("Server ended"),
    }

    Ok(())
}

fn init_logging() {
    let file_number = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // We don't care if this fails because of a missing file. This could fail other ways, but I'm not handling that right now.
    let _move_result = fs::rename("log.txt", format!("log{file_number}.txt"));
    let log_file = File::create("log.txt").unwrap();

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .with_filter(EnvFilter::from_default_env());

    let file_layer = layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_filter(LevelFilter::WARN);

    Registry::default()
        .with(stdout_layer)
        .with(file_layer)
        .try_init()
        .unwrap();

    trace!("Initialized logging");
}

async fn start_server() -> (InfoChannel, Serve<TcpListener, Router, Router>) {
    let server = Server::default();
    server.run().await
}
