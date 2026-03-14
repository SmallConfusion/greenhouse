use crate::peripheral::{peripheral_command::PeripheralCommand, peripheral_trait::CommandPreset};
use derive_more::{Deref, From};
use schemars::JsonSchema;
use serde::Deserialize;
use std::ops::Range;

trait GenericCommand {
    fn send(&mut self);
}

impl<T: PeripheralCommand> GenericCommand for CommandPreset<T> {
    fn send(&mut self) {
        self.send()
    }
}

pub trait Condition {
    fn can_enter(&self) -> bool;
    fn can_exit(&self) -> bool;
    fn needs_exit(&self) -> bool;
}

pub struct Controller {
    stage_sets: Vec<StageSet>,
}

pub struct StageSet {
    set: Vec<Stage>,
}

pub struct Stage {
    entry: Vec<Box<dyn GenericCommand>>,
    condition: Box<dyn Condition>,
}

pub struct TempRange {
    range: Range<f32>,
}

#[derive(Debug, Deserialize, JsonSchema, From, Deref)]
pub struct VentState(pub f32);
