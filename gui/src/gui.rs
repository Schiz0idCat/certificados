use crate::templates::Assistance;

use eframe::egui;

pub struct Gui;

impl Gui {
    pub fn run() -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([400.0, 500.0])
                .with_resizable(false),
            ..Default::default()
        };

        eframe::run_native(
            "Generador de Certificados",
            options,
            Box::new(|_cc| Ok(Box::<Assistance>::default())),
        )
    }
}
