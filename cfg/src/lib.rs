use rut::DisplayOpt;

use std::sync::OnceLock;

pub struct Cfg {
    date_fmt: &'static str,
    time_fmt: &'static str,
    rut_fmt: DisplayOpt,
}

impl Cfg {
    pub fn global() -> &'static Self {
        static INSTANCE: OnceLock<Cfg> = OnceLock::new();
        INSTANCE.get_or_init(|| Cfg::default())
    }

    pub fn date_fmt(&self) -> &'static str {
        self.date_fmt
    }

    pub fn time_fmt(&self) -> &'static str {
        self.time_fmt
    }

    pub fn rut_fmt(&self) -> DisplayOpt {
        self.rut_fmt
    }
}

impl Default for Cfg {
    fn default() -> Self {
        Self {
            date_fmt: "%d/%m/%Y",
            time_fmt: "%H:%M",
            rut_fmt: DisplayOpt::default(),
        }
    }
}
