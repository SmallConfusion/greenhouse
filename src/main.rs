pub mod condition;
pub mod config;
pub mod controller;
pub mod input;
pub mod peripheral;

use crate::{
    config::{Config, args::parse_args},
    input::temperature::init_temperature,
};
use color_eyre::eyre::Result;
use schemars::schema_for;
use std::{
    fs::File,
    io::Write as _,
    time::{SystemTime, UNIX_EPOCH},
};
use tracing::{info, level_filters::LevelFilter, trace};
use tracing_subscriber::{
    EnvFilter, Layer as _, Registry, fmt::layer, layer::SubscriberExt as _,
    util::SubscriberInitExt as _,
};

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
    controller.run().await;

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
