use schemars::JsonSchema;
use serde::Deserialize;
use std::{collections::HashMap, ops::Range};

use crate::{controller::VentState, pin::PinState};

#[derive(Debug, Deserialize, JsonSchema)]
pub struct ControllerDesc {
    pub peripherals: HashMap<String, PeripheralDesc>,
    pub stage_sets: Vec<StageSetDesc>,
}

#[derive(Debug, Deserialize, JsonSchema)]
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
pub enum SettingDesc {
    Pin(PinState),
    Vent(VentState),
}

#[derive(Debug, Deserialize, JsonSchema)]
pub enum ConditionDesc {
    TempRange(Range<f32>),
}
