#![allow(dead_code)]

pub mod controller;
pub mod convert;
pub mod description;
pub mod peripheral;
pub mod pin;

use color_eyre::eyre::Result;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::FmtSubscriber;

fn main() -> Result<()> {
    color_eyre::install()?;
    init_logging();

    Ok(())
}

fn init_logging() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(LevelFilter::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Initialized logging");
}
