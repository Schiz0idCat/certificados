use cfg::Cfg;
use eframe::egui;
use time::OffsetDateTime;

pub struct Assistance {
    name: String,
    birth: String,
    rut: String,
    today: String,
    appointment: String,
    start_time: String,
    end_time: String,
}

impl Default for Assistance {
    fn default() -> Self {
        let date_fmt = Cfg::global().date_fmt();

        Self {
            name: String::new(),
            birth: String::new(),
            rut: String::new(),
            today: OffsetDateTime::now_utc().date().format(date_fmt).unwrap(),
            appointment: String::new(),
            start_time: String::new(),
            end_time: String::new(),
        }
    }
}

impl eframe::App for Assistance {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ui_builder = egui::UiBuilder::new();

        ui.scope_builder(ui_builder, |ui| {
            ui.vertical_centered(|ui| {
                ui.set_max_width(450.0);

                ui.add_space(6.0);
                ui.heading("Generar Certificado de Asistencia");
                ui.separator();

                ui.add_space(12.0);

                egui::Frame::NONE
                    .inner_margin(egui::Margin::symmetric(20, 0))
                    .show(ui, |ui| {
                        egui::Grid::new("assistance_grid")
                            .num_columns(2)
                            .spacing([40.0, 12.0])
                            .striped(true)
                            .min_col_width(ui.available_width() / 2.5)
                            .show(ui, |ui| {
                                ui.label("Nombre:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.name)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("RUT:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.rut)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Fecha de Nacimiento:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.birth)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Fecha Informe:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.today)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Fecha Cita:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.appointment)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Hora Inicio:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.start_time)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();

                                ui.label("Hora Fin:");
                                ui.add(
                                    egui::TextEdit::singleline(&mut self.end_time)
                                        .desired_width(f32::INFINITY),
                                );
                                ui.end_row();
                            });
                    });

                ui.add_space(24.0);
                ui.separator();
                ui.add_space(12.0);

                let button = ui.add_sized([120.0, 40.0], egui::Button::new("Generar PDF"));

                if button.clicked() {
                    println!("Certificado de asistencia para {}", self.name);
                }
            });
        });
    }
}
