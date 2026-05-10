use crate::errors::AssistanceGuiError;
use pdf::errors::BrowserError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GuiError {
    #[error("{0}")]
    BrowserError(#[from] BrowserError),

    #[error("{0}")]
    AssistanceGuiError(#[from] AssistanceGuiError),
}
