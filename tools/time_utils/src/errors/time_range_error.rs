use thiserror::Error;
use time::Time;

#[derive(Debug, Error)]
pub enum TimeRangeError {
    #[error("Start time ({0}) must be before end time ({0}).")]
    InvalidRange(Time, Time),
}
