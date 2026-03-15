pub mod controller;
pub mod convert;
pub mod description;
pub mod peripheral;
pub mod peripherals;

use crate::{convert::convert_controller, description::ControllerDesc};
use color_eyre::eyre::Result;
use std::time::Duration;
use tracing::{info, level_filters::LevelFilter};
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

    tokio::time::sleep(Duration::from_secs(10)).await;

    Ok(())
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Initialized logging");
}

mod condition {
    pub trait Condition {
        // TODO: Move to module
        fn is_met(&self) -> bool;
    }
}
