use crate::templates::Assistance;

use eframe::egui;

pub struct Gui;

impl Gui {
    pub fn run() -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([600.0, 340.0])
                .with_resizable(false),
            ..Default::default()
        };

        eframe::run_native(
            "Generador de Certificados",
            options,
            Box::new(|cc| {
                let mut style = (*cc.egui_ctx.global_style()).clone();

                style.text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                );

                style.text_styles.insert(
                    egui::TextStyle::Monospace,
                    egui::FontId::new(20.0, egui::FontFamily::Monospace),
                );

                cc.egui_ctx.set_global_style(style);

                Ok(Box::<Assistance>::default())
            }),
        )
    }
}
