use crate::errors::TimeRangeError;
use jiff::civil::Time;

pub struct TimeRange {
    start: Time,
    end: Time,
}

impl TimeRange {
    pub fn try_new(start: Time, end: Time) -> Result<Self, TimeRangeError> {
        if start > end {
            return Err(TimeRangeError::InvalidRange(start, end));
        }

        Ok(Self { start, end })
    }

    pub fn start(&self) -> Time {
        self.start
    }

    pub fn end(&self) -> Time {
        self.end
    }
}
