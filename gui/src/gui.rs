use crate::templates::Assistance;

use eframe::egui;

pub struct Gui;

impl Gui {
    pub fn run() -> eframe::Result {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([570.0, 390.0])
                .with_resizable(false),
            ..Default::default()
        };

        eframe::run_native(
            "Generador de Certificados",
            options,
            Box::new(|cc| {
                let mut visuals = egui::Visuals::dark();

                visuals.extreme_bg_color = egui::Color32::from_gray(2);

                cc.egui_ctx.set_visuals(visuals);

                let mut style = (*cc.egui_ctx.global_style()).clone();

                style.text_styles.insert(
                    egui::TextStyle::Heading,
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    egui::TextStyle::Body,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                );
                style.text_styles.insert(
                    egui::TextStyle::Button,
                    egui::FontId::new(16.0, egui::FontFamily::Proportional),
                );

                cc.egui_ctx.set_global_style(style);

                Ok(Box::<Assistance>::default())
            }),
        )
    }
}
