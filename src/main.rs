pub mod condition;
pub mod controller;
pub mod convert;
pub mod description;
pub mod peripheral;

use std::{fs::File, io::Write};

use crate::{convert::convert_controller, description::ControllerDesc};
use clap::Parser;
use color_eyre::eyre::Result;
use schemars::schema_for;
use tracing::{error, info, level_filters::LevelFilter, trace};
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, Parser)]
struct Args {
    #[arg(short, long)]
    schema: bool,

    #[arg(short, long)]
    template: bool,

    #[arg(short, long, default_value_t = String::from("config.yaml"))]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();

    let args = Args::parse();

    if args.schema {
        info!("Writing schema file to config.schema.json");
        let schema = serde_json::to_string(&schema_for!(ControllerDesc))?;
        write!(File::create("config.schema.json")?, "{schema}")?;
    }

    if args.template {
        info!("Writing template config file to config.yaml");
        let content = include_str!("config.yaml");
        write!(File::create_new("config.yaml")?, "{content}")?;
    }

    if args.schema || args.template {
        info!("Did config stuff, not running. Run without -s or -t to run controller");
        return Ok(());
    }

    info!("Loading config from {}", args.config);
    let config = std::fs::read_to_string(args.config)?;
    let desc: ControllerDesc = serde_yaml::from_str(&config)?;

    let c = convert_controller(desc);
    c.run().await;

    Ok(())
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    trace!("Initialized logging");
}

pub fn get_temperature() -> f32 {
    todo!();

    loop {
        let temp_str = std::fs::read_to_string("temp.txt").unwrap();
        let Ok(r) = temp_str.trim().parse() else {
            error!("Incorrect temperature {temp_str}");
            continue;
        };

        trace!("Got temperature {r}");

        return r;
    }
}
