use std::ops::Range;

use derive_more::Constructor;

use crate::condition::Condition;
use crate::input::get_temperature;

#[derive(Debug, Constructor)]
pub struct TemperatureRange {
    range: Range<f32>,
}

impl Condition for TemperatureRange {
    fn is_met(&self) -> bool {
        self.range.contains(&get_temperature())
    }
}
