pub mod args;
pub mod convert;
pub mod description;

use std::{fs::File, io::BufReader};

use crate::config::description::ControllerDesc;
use schemars::JsonSchema;
use serde::Deserialize;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct Config {
    pub controller_desc: ControllerDesc,
    pub temperature_path: String,
}

impl Config {
    /// # Panics
    ///
    /// Will panic if file donsn't exist or if config file is invalid.
    pub fn load(path: &str) -> Self {
        let file = File::open(path).expect("Can't open config file");
        let reader = BufReader::new(file);
        serde_yaml::from_reader(reader).expect("Can't parse config file")
    }
}
