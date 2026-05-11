use rut::error::RutError;
use templates::errors::AssistanceError;
use thiserror::Error;
use time_utils::errors::TimeRangeError;

#[derive(Debug, Error)]
pub enum AssistanceGuiError {
    #[error("Time error: {0}.")]
    TimeRange(#[from] TimeRangeError),

    #[error("Template error: {0}.")]
    Template(#[from] AssistanceError),

    #[error("Name cannot be empty.")]
    EmptyName,

    #[error("Rut error: {0}.")]
    InvalidRut(#[from] RutError),
}
