use crate::peripheral::peripheral_command::AnyCommand;
use crate::{
    condition::{implementation::temperature::TemperatureRange, Condition},
    peripheral::{
        implementation::{
            pin::{Pin, PinState},
            vent::{Vent, VentState},
        },
        running_peripheral::{GenericPeripheral, RunningPeripheral},
        Peripheral,
    },
};
use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, ops::Range};
use tokio::task::JoinHandle;

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControllerDesc {
    pub peripherals: HashMap<String, PeripheralDesc>,
    pub stage_sets: Vec<StageSetDesc>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum PeripheralDesc {
    Pin(u8, PinState),
    Vent(VentDesc, VentState),
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct VentDesc {
    pub on: u8,
    pub off: u8,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StageSetDesc {
    pub stages: Vec<StageDesc>,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct StageDesc {
    pub settings: HashMap<String, SettingDesc>,
    pub condition: ConditionDesc,
    pub exit_condition: Option<ConditionDesc>,
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum SettingDesc {
    Pin(PinState),
    Vent(VentState),
}

#[derive(Debug, Deserialize, JsonSchema)]
#[serde(untagged)]
pub enum ConditionDesc {
    TempRange(Range<f32>),
}

impl PeripheralDesc {
    pub fn to_generic_real(self) -> (Box<dyn GenericPeripheral>, JoinHandle<()>) {
        match self {
            Self::Pin(pin, pin_state) => Self::create_generic_peripheral(Pin::new(pin), pin_state),

            Self::Vent(VentDesc { on, off }, vent_state) => {
                Self::create_generic_peripheral(Vent::new(Pin::new(on), Pin::new(off)), vent_state)
            }
        }
    }

    fn create_generic_peripheral<T: Peripheral>(
        peripheral: T,
        default: T::Command,
    ) -> (Box<dyn GenericPeripheral>, JoinHandle<()>) {
        let (running, join) = RunningPeripheral::create_from_peripheral(peripheral, default);
        let generic = Box::new(running);
        (generic, join)
    }
}

impl SettingDesc {
    pub fn into_any(self) -> AnyCommand {
        match self {
            Self::Pin(inner) => AnyCommand::new(inner),
            Self::Vent(inner) => AnyCommand::new(inner),
        }
    }
}

impl ConditionDesc {
    pub fn into_generic(self) -> Box<dyn Condition> {
        match self {
            Self::TempRange(range) => Box::new(TemperatureRange::new(range)),
        }
    }
}
