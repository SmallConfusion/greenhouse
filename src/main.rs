#![allow(dead_code)]

pub mod controller;
pub mod convert;
pub mod description;
pub mod peripheral;
pub mod peripherals;

use std::{collections::HashMap, time::Duration};

use color_eyre::eyre::Result;
use schemars::{SchemaGenerator, schema_for};
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::FmtSubscriber;

use crate::{
    controller::{Controller, StageSet, stage::Stage},
    description::{ControllerDesc, PeripheralDesc, SettingDesc, StageDesc, StageSetDesc, VentDesc},
    peripherals::{
        pin::{self, PinState},
        vent::VentState,
    },
};

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

    let c = Controller::convert(desc);

    tokio::time::sleep(Duration::from_secs(1)).await;

    dbg!(c);

    tokio::time::sleep(Duration::from_secs(1)).await;

    Ok(())
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Initialized logging");
}
