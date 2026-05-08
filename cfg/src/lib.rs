use time::format_description::FormatItem;
use time::macros::format_description;

use std::sync::OnceLock;

pub struct Cfg {
    date_fmt: &'static [FormatItem<'static>],
    time_fmt: &'static [FormatItem<'static>],
}

impl Cfg {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Cfg> = OnceLock::new();
        INSTANCE.get_or_init(|| Cfg::default())
    }

    pub fn date_fmt(&self) -> &'static [FormatItem<'static>] {
        self.date_fmt
    }

    pub fn time_fmt(&self) -> &'static [FormatItem<'static>] {
        self.time_fmt
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            date_fmt: format_description!("[day]-[month]-[year]"),
            time_fmt: format_description!("[hour]:[minute]"),
        }
    }
}
