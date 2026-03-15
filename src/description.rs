use schemars::JsonSchema;
use serde::Deserialize;
use std::{any::Any, collections::HashMap, ops::Range};
use tokio::task::JoinHandle;

use crate::{
    peripheral::{
        Peripheral,
        running_peripheral::{GenericPeripheral, RunningPeripheral},
    },
    peripherals::{
        pin::{Pin, PinState},
        vent::{Vent, VentState},
    },
};

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
            PeripheralDesc::Pin(pin, pin_state) => {
                Self::create_generic_peripheral(Pin::new(pin), pin_state)
            }

            PeripheralDesc::Vent(VentDesc { on, off }, vent_state) => {
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
    pub fn into_any(self) -> Box<dyn Any> {
        match self {
            SettingDesc::Pin(v) => Box::new(v),
            SettingDesc::Vent(v) => Box::new(v),
        }
    }
}
