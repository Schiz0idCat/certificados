use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssistanceError {
    #[error("Browser error: {0}")]
    Browser(#[from] pdf::errors::BrowserError),

    #[error("time error: {0}")]
    Format(#[from] jiff::Error),

    #[error("Birth after appointment.")]
    BirthAfterAppointment,

    #[error("Birth after today.")]
    BirthAfterToday,
}
