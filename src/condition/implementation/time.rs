use time::{OffsetDateTime, Time};

use crate::condition::Condition;
use crate::config::description::TimeRangeDesc;

#[derive(Debug)]
pub struct TimeRange {
    start: Time,
    end: Time,
}

impl Condition for TimeRange {
    fn is_met(&self) -> bool {
        let now = OffsetDateTime::now_local()
            .expect("Local time zones broken!")
            .time();

        let inside = self.start <= now && now < self.end;
        let outside = self.end <= now && now < self.start;

        inside || outside
    }
}

impl TryFrom<TimeRangeDesc> for TimeRange {
    type Error = time::error::ComponentRange;

    fn try_from(value: TimeRangeDesc) -> Result<Self, Self::Error> {
        let start = Time::from_hms(value.start.hours, value.start.minutes, 0)?;
        let end = Time::from_hms(value.end.hours, value.end.minutes, 0)?;

        Ok(Self { start, end })
    }
}
