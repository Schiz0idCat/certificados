use eframe::egui;
use jiff::civil::Date;

pub struct Assistance {
    name: String,
    birth: Date,
    rut: String,
    today: Date,
    appointment: Date,
    start_time: String,
    end_time: String,
}

impl Default for Assistance {
    fn default() -> Self {
        let today = jiff::Zoned::now().date();

        Self {
            name: String::new(),
            birth: today,
            rut: String::new(),
            today: today,
            appointment: today,
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
                ui.set_max_width(505.0);

                ui.add_space(10.0);
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
                                spanish_date_picker(ui, "birth_picker", &mut self.birth);
                                ui.end_row();

                                ui.label("Fecha Informe:");
                                spanish_date_picker(ui, "today_picker", &mut self.today);
                                ui.end_row();

                                ui.label("Fecha Cita:");
                                spanish_date_picker(ui, "apmt_picker", &mut self.appointment);
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

const MESES: [&str; 12] = [
    "Enero",
    "Febrero",
    "Marzo",
    "Abril",
    "Mayo",
    "Junio",
    "Julio",
    "Agosto",
    "Septiembre",
    "Octubre",
    "Noviembre",
    "Diciembre",
];

fn spanish_date_picker(ui: &mut egui::Ui, id: &str, date: &mut jiff::civil::Date) {
    let mut d = date.day() as i32;
    let mut m = date.month() as i32;
    let mut y = date.year() as i32;

    // Lógica para actualizar la fecha ajustando el día si el mes es más corto
    let mut apply_change = |new_y: i32, new_m: i32, new_d: i32| {
        if let Some(nd) = jiff::civil::Date::new(new_y as i16, new_m as i8, new_d as i8).ok() {
            *date = nd;
        } else {
            // Si falla (ej: 31 de Febrero), buscamos el último día válido de ese mes
            let temp_date = jiff::civil::Date::new(new_y as i16, new_m as i8, 1).unwrap();
            let last_day = temp_date.days_in_month() as i32;
            if let Some(nd) = jiff::civil::Date::new(new_y as i16, new_m as i8, last_day as i8).ok()
            {
                *date = nd;
            }
        }
    };

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 4.0;

        // --- DÍA ---
        let day_cb = egui::ComboBox::from_id_salt(format!("{}_d", id))
            .selected_text(format!("{:02}", d))
            .width(45.0)
            .show_ui(ui, |ui| {
                for i in 1..=31 {
                    if ui.selectable_value(&mut d, i, i.to_string()).changed() {
                        apply_change(y, m, d);
                    }
                }
            });

        // --- MES ---
        let month_cb = egui::ComboBox::from_id_salt(format!("{}_m", id))
            .selected_text(MESES[(m - 1) as usize])
            .width(110.0)
            .show_ui(ui, |ui| {
                for (i, nombre) in MESES.iter().enumerate() {
                    let val = (i + 1) as i32;
                    if ui.selectable_value(&mut m, val, *nombre).changed() {
                        apply_change(y, m, d);
                    }
                }
            });

        // --- AÑO ---
        let year_cb = egui::ComboBox::from_id_salt(format!("{}_y", id))
            .selected_text(y.to_string())
            .width(75.0)
            .show_ui(ui, |ui| {
                for i in (1900..=2100).rev() {
                    if ui.selectable_value(&mut y, i, i.to_string()).changed() {
                        apply_change(y, m, d);
                    }
                }
            });

        // --- LÓGICA DE SCROLL (1 EN 1) ---
        let mut step = 0;
        ui.input(|i| {
            for event in &i.raw.events {
                if let egui::Event::MouseWheel { delta, .. } = event {
                    if delta.y > 0.0 {
                        step = 1;
                    } else if delta.y < 0.0 {
                        step = -1;
                    }
                }
            }
        });

        if step != 0 {
            if day_cb.response.hovered() {
                let new_d = (d + step - 1).rem_euclid(31) + 1;
                apply_change(y, m, new_d);
            } else if month_cb.response.hovered() {
                let new_m = (m + step - 1).rem_euclid(12) + 1;
                apply_change(y, new_m, d);
            } else if year_cb.response.hovered() {
                apply_change(y + step, m, d);
            }

            ui.input_mut(|i| i.smooth_scroll_delta = egui::Vec2::ZERO);
        }
    });
}
