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
                ui.heading("Generar Certificado de Asistencia");
            });

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(12.0);

            egui::Grid::new("assistance_grid")
                .num_columns(2)
                .spacing([40.0, 12.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Nombre:");
                    ui.text_edit_singleline(&mut self.name);
                    ui.end_row();

                    ui.label("RUT:");
                    ui.text_edit_singleline(&mut self.rut);
                    ui.end_row();

                    ui.label("Fecha de Nacimiento:");
                    ui.text_edit_singleline(&mut self.birth);
                    ui.end_row();

                    ui.label("Fecha Informe:");
                    ui.text_edit_singleline(&mut self.today);
                    ui.end_row();

                    ui.label("Fecha Cita:");
                    ui.text_edit_singleline(&mut self.appointment);
                    ui.end_row();

                    ui.label("Hora Inicio:");
                    ui.text_edit_singleline(&mut self.start_time);
                    ui.end_row();

                    ui.label("Hora Fin:");
                    ui.text_edit_singleline(&mut self.end_time);
                    ui.end_row();
                });

            ui.add_space(24.0);
            ui.separator();
            ui.add_space(12.0);

            ui.vertical_centered(|ui| {
                let button = ui.add_sized([120.0, 40.0], egui::Button::new("Generar PDF"));

                if button.clicked() {
                    println!("Certificado de asistencia para {}", self.name);
                }
            });
        });
    }
}
