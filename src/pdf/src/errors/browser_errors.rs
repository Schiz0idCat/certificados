pub use thiserror::Error;

#[derive(Debug, Error)]
pub enum BrowserError {
    #[error("Couldn't render the html file.")]
    Render,

    #[error("Browser couldn't open.")]
    OpenBrowser,

    #[error("Browser couldn't open a tab.")]
    OpenTab,

    #[error("Couldn't navigate throght tabs.")]
    TabNavigation,

    #[error("Couldn't print.")]
    Print,

    #[error("Couldn't save the pdf.")]
    Io,
}
