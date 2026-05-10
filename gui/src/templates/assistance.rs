use crate::widgets::{Calendar, TimeRange};
use rut::Rut;

use eframe::egui;
use jiff::Zoned;
use jiff::civil::{Date, Time};

pub struct Assistance {
    name: String,
    birth: Date,
    rut: Rut,
    rut_buf: String,
    today: Date,
    appointment: Date,
    start_time: Time,
    end_time: Time,
}

impl Default for Assistance {
    fn default() -> Self {
        let today = Zoned::now().date();
        let rut = Rut::try_new(11111111, '1').unwrap();

        Self {
            name: String::new(),
            birth: today,
            rut_buf: String::new(),
            rut: rut,
            today: today,
            appointment: today,
            start_time: Zoned::now().time(),
            end_time: Zoned::now().time(),
        }
    }
}

impl eframe::App for Assistance {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Generar Certificado de Asistencia");
                ui.separator();

                egui::Frame::NONE
                    .inner_margin(egui::Margin::symmetric(20, 0))
                    .show(ui, |ui| {
                        egui::Grid::new("assistance_grid")
                            .num_columns(2)
                            .spacing([40.0, 12.0])
                            .min_col_width(ui.available_width() / 5.0)
                            .striped(true)
                            .show(ui, |ui| {
                                Self::text(ui, "Nombre:", &mut self.name);
                                Self::rut(ui, "RUT:", &mut self.rut_buf, &mut self.rut);

                                Self::date(ui, "Fecha de Nacimiento:", "birth", &mut self.birth);
                                Self::date(ui, "Fecha Informe:", "today", &mut self.today);
                                Self::date(ui, "Fecha Cita:", "apmt", &mut self.appointment);

                                Self::time(
                                    ui,
                                    "Horario:",
                                    "range_p",
                                    &mut self.start_time,
                                    &mut self.end_time,
                                );
                            });
                    });

                ui.separator();

                let button = ui.add_sized([120.0, 40.0], egui::Button::new("Generar PDF"));
                if button.clicked() {
                    // Acción
                }
            });
        });
    }
}

impl Assistance {
    fn text(ui: &mut egui::Ui, lbl: &str, var: &mut String) {
        ui.label(lbl);
        ui.add(egui::TextEdit::singleline(var).desired_width(f32::INFINITY));
        ui.end_row();
    }

    fn date(ui: &mut egui::Ui, lbl: &str, id: &str, var: &mut Date) {
        ui.label(lbl);
        Calendar::show(ui, id, var);
        ui.end_row();
    }

    fn time(ui: &mut egui::Ui, lbl: &str, id: &str, start: &mut Time, end: &mut Time) {
        ui.label(lbl);
        TimeRange::show(ui, id, start, end);
        ui.end_row();
    }

    fn rut(ui: &mut egui::Ui, lbl: &str, buffer: &mut String, rut_val: &mut Rut) {
        ui.label(lbl);

        ui.scope(|ui| {
            let is_valid = buffer.is_empty() || buffer.parse::<Rut>().is_ok();

            if !is_valid {
                let visuals = &mut ui.style_mut().visuals.widgets;
                visuals.inactive.bg_stroke = egui::Stroke::new(1.0, egui::Color32::RED);
                visuals.hovered.bg_stroke = egui::Stroke::new(1.0, egui::Color32::RED);
                visuals.active.bg_stroke = egui::Stroke::new(1.0, egui::Color32::RED);
            }

            let res = ui.add(egui::TextEdit::singleline(buffer).desired_width(f32::INFINITY));

            if res.changed() {
                if let Ok(valid_rut) = buffer.parse::<Rut>() {
                    *rut_val = valid_rut;
                }
            }

            if res.lost_focus() {
                if let Ok(valid_rut) = buffer.parse::<Rut>() {
                    *buffer = valid_rut.to_string();
                }
            }
        });

        ui.end_row();
    }
}
