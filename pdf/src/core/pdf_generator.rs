use crate::errors::BrowserError;

use askama::Template;
use headless_chrome::{Browser, LaunchOptions};
use std::fs;
use std::path::{Path, PathBuf};

pub trait PdfGenerator: Template {
    fn save_as_pdf(&self, path: &Path) -> Result<(), BrowserError> {
        let html = self.render().map_err(|_| BrowserError::Render)?;

        let opt = LaunchOptions {
            path: find_chrome_bin(),
            ..Default::default()
        };

        let browser = Browser::new(opt).map_err(|_| BrowserError::OpenBrowser)?;
        let tab = browser.new_tab().map_err(|_| BrowserError::OpenTab)?;

        let data_url = format!(
            "data:text/html;charset=utf-8,{}",
            urlencoding::encode(&html)
        );
        tab.navigate_to(&data_url)
            .map_err(|_| BrowserError::TabNavigation)?;
        tab.wait_until_navigated()
            .map_err(|_| BrowserError::TabNavigation)?;

        let pdf_data = tab.print_to_pdf(None).map_err(|_| BrowserError::Print)?;

        fs::write(path, pdf_data).map_err(|_| BrowserError::Io)?;

        Ok(())
    }
}

fn find_chrome_bin() -> Option<PathBuf> {
    let names = [
        "google-chrome",
        "chromium",
        "brave",
        "microsoft-edge",
        "opera",
    ];
    let path_var = std::env::var_os("PATH")?;

    for directory in std::env::split_paths(&path_var) {
        for name in names {
            let bin_path = directory.join(name);

            if bin_path.exists() {
                return Some(bin_path);
            }
        }
    }

    None
}
