pub use thiserror::Error;

#[derive(Debug, Error)]
pub enum AssistanceError {
    #[error("Start time must be before end time.")]
    Time,

    #[error("Browser error: {0}")]
    Browser(#[from] pdf::errors::BrowserError),
}
