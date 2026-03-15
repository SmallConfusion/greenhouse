pub mod condition;
pub mod controller;
pub mod convert;
pub mod description;
pub mod peripheral;

use crate::{convert::convert_controller, description::ControllerDesc};
use color_eyre::eyre::Result;
use std::{io::Read, time::Duration};
use tracing::{error, info, level_filters::LevelFilter, trace};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();

    let config = std::fs::read_to_string("config.yaml").unwrap();
    let desc: ControllerDesc = serde_yaml::from_str(&config).unwrap();

    dbg!(&desc);

    // println!(
    //     "{}",
    //     serde_json::to_string(&schema_for!(ControllerDesc)).unwrap()
    // );

    let c = convert_controller(desc);
    c.run().await;

    Ok(())
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Initialized logging");
}

pub fn get_temperature() -> f32 {
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
