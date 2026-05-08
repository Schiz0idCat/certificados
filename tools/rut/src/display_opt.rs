#[derive(Clone, Copy)]
pub struct DisplayOpt {
    dots: bool,
    hyphen: bool,
}

impl DisplayOpt {
    pub fn new(dots: bool, hyphen: bool) -> Self {
        Self { dots, hyphen }
    }

    pub fn dots(&self) -> bool {
        self.dots
    }

    pub fn hyphen(&self) -> bool {
        self.hyphen
    }
}

impl Default for DisplayOpt {
    fn default() -> Self {
        Self::new(true, true)
    }
}
