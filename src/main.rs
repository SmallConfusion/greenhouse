pub mod condition;
pub mod config;
pub mod controller;
pub mod input;
pub mod peripheral;
pub mod web_server;

use std::fs::File;
use std::io::Write as _;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Router;
use axum::serve::Serve;
use color_eyre::eyre::Result;
use schemars::schema_for;
use tokio::net::TcpListener;
use tokio::select;
use tracing::level_filters::LevelFilter;
use tracing::{info, trace};
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt as _;
use tracing_subscriber::util::SubscriberInitExt as _;
use tracing_subscriber::{EnvFilter, Layer as _, Registry};
use web_server::data::InfoChannel;

use crate::config::Config;
use crate::config::args::parse_args;
use crate::input::temperature::init_temperature;
use crate::web_server::server::Server;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();

    let args = parse_args();

    if args.schema | args.template {
        info!("Writing schema file to config.schema.json");
        let schema = serde_json::to_string(&schema_for!(Config))?;
        write!(File::create("config.schema.json")?, "{schema}")?;
        println!("Wrote schema file to config.schema.json");
    }

    if args.template {
        info!("Writing template config file to config.yaml");
        let content = include_str!("config/config.yaml");
        write!(File::create_new("config.yaml")?, "{content}")?;
        println!("Wrote template config file to config.yaml");
    }

    if args.schema || args.template {
        println!("Did config stuff, not running. Run without -s or -t to run controller");
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
        () = controller_join => (), // TODO: Add useful info logging here
        _ = server_join => (),
    }

    Ok(())
}

fn init_logging() {
    let file_number = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let log_file = File::create(format!("log{file_number}.txt")).unwrap();

    let stdout_layer = tracing_subscriber::fmt::layer()
        .with_line_number(true)
        .with_file(true)
        .pretty()
        .with_filter(EnvFilter::from_default_env());

    let file_layer = layer()
        .with_writer(log_file)
        .with_ansi(false)
        .with_filter(LevelFilter::INFO);

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
