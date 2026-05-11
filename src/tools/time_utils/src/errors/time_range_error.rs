use jiff::civil::Time;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TimeRangeError {
    #[error("Start time ({0}) must be before end time ({0}).")]
    InvalidRange(Time, Time),
}
