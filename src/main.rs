use log::error;
use rppal::gpio::Gpio;
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use std::{
    fs::File,
    io::{self, Write},
    path::Path,
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn main() {
    init_logging();

    let Some(config) = init_config() else {
        return;
    };

    loop {
        config.tick();
    }
}

fn init_logging() {
    let time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis();

    let filename = format!("log{}.txt", time);
    // let filename = "log.txt";

    simple_logging::log_to_file(filename, log::LevelFilter::Debug).unwrap();

    log::debug!("Initialized logging");
}

fn init_config() -> Option<Config> {
    let schema = schema_for!(Config);
    let schema_text = serde_json::to_string_pretty(&schema).unwrap();

    let schema_filename = "config.schema.json";
    let mut schema_file = File::create(schema_filename).unwrap();
    write!(schema_file, "{schema_text}").unwrap();

    log::debug!("Wrote schema file");

    let config_filename = "config.yaml";
    if !Path::new(config_filename).exists() {
        let config_contents = "# yaml-language-server: $schema=config.schema.json\n";
        let mut config_file = File::create(config_filename).unwrap();
        write!(config_file, "{config_contents}").unwrap();

        log::debug!("No config file found, template written.");

        None
    } else {
        let config_file = File::open(config_filename).unwrap();
        let config = serde_yaml::from_reader(config_file).unwrap();

        log::debug!("Config loaded: {config:?}");

        Some(config)
    }
}

#[derive(Debug, Clone, JsonSchema, Deserialize)]
pub struct Config {
    pub stage_progressions: Vec<StageProcession>,
    pub pin_defaults: PinSet,
}

#[derive(Debug, Clone, PartialEq, JsonSchema, Deserialize)]
pub struct PinSet(pub Vec<(Pin, bool)>);

#[derive(Debug, Clone, PartialEq, JsonSchema, Deserialize)]
pub struct StageProcession {
    pub stages: Vec<Stage>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, JsonSchema, Deserialize)]
pub enum Comparison {
    Greater,
    Less,
}

#[derive(Debug, Clone, PartialEq, JsonSchema, Deserialize)]
pub struct Stage {
    pub name: String,
    pub comparator: Comparison,
    pub on_temp: i32,
    pub off_temp: i32,
    pub minimum_time: u32,
    pub pins: PinSet,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, JsonSchema, Deserialize)]
pub struct Pin(pub u8);

impl Pin {
    pub fn on(&self) -> anyhow::Result<()> {
        Gpio::new()?.get(self.0)?.into_output_high();
        Ok(())
    }

    pub fn off(&self) -> anyhow::Result<()> {
        Gpio::new()?.get(self.0)?.into_output_low();
        Ok(())
    }
}

impl PinSet {
    pub fn activate(&self) -> anyhow::Result<()> {
        for (pin, state) in self.0.iter() {
            if *state {
                pin.on()?;
            } else {
                pin.off()?;
            }
        }

        Ok(())
    }
}

impl Stage {
    pub fn should_enter(&self, temp: i32) -> bool {
        match self.comparator {
            Comparison::Greater => temp >= self.on_temp,
            Comparison::Less => temp <= self.on_temp,
        }
    }

    pub fn can_exit(&self, temp: i32) -> bool {
        match self.comparator {
            Comparison::Greater => temp < self.off_temp,
            Comparison::Less => temp > self.off_temp,
        }
    }

    pub fn activate(&self) -> anyhow::Result<()> {
        log::info!("Activating state {}", self.name);
        self.pins.activate()?;
        thread::sleep(Duration::from_secs(self.minimum_time as u64));
        Ok(())
    }
}

impl StageProcession {
    pub fn try_activate(&self, temp: i32) -> bool {
        let mut index = 0;

        if !self.stages.get(0).map_or(false, |s| s.should_enter(temp)) {
            return false;
        }

        log_and_ignore(self.stages[index].activate());

        loop {
            let temp = get_temperature();

            if self
                .stages
                .get(index + 1)
                .map_or(false, |s| s.should_enter(temp))
            {
                index += 1;
                log_and_ignore(self.stages[index].activate());
            } else if self.stages[index].can_exit(temp) {
                if index == 0 {
                    return true;
                }

                index -= 1;
                log_and_ignore(self.stages[index].activate());
            }

            thread::sleep(Duration::from_secs(1));
        }
    }
}

impl Config {
    pub fn reset_state(&self) {
        log::info!("Resetting to base state.");
        log_and_ignore(self.pin_defaults.activate());
    }

    pub fn tick(&self) {
        let temp = get_temperature();

        let mut any_activated = false;

        for p in self.stage_progressions.iter() {
            if p.try_activate(temp) {
                any_activated = true;
                break;
            }
        }

        if any_activated {
            self.reset_state();
        }

        thread::sleep(Duration::from_secs(1));
    }
}

pub fn get_temperature() -> i32 {
    // log::error!("Get temperature not implemented yet");

    loop {
        print!("Temperature requested: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();

        match buffer.trim().parse() {
            Ok(temp) => return temp,
            Err(err) => println!("{err}"),
        }
    }
}

pub fn log_and_ignore(res: anyhow::Result<()>) {
    if let Err(err) = res {
        error!("{err}");
    }
}
