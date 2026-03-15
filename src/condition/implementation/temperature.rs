use crate::{condition::Condition, get_temperature};
use derive_more::Constructor;
use std::ops::Range;

#[derive(Debug, Constructor)]
pub struct TemperatureRange {
    range: Range<f32>,
}

impl Condition for TemperatureRange {
    fn is_met(&self) -> bool {
        self.range.contains(&get_temperature())
    }
}
