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
                                let text_row = |ui: &mut egui::Ui, lbl: &str, var: &mut String| {
                                    ui.label(lbl);
                                    ui.add(egui::TextEdit::singleline(var).desired_width(f32::INFINITY));
                                    ui.end_row();
                                };

                                let  date_row = |ui: &mut egui::Ui, lbl: &str, id: &str, var: &mut jiff::civil::Date| {
                                    ui.label(lbl);
                                    spanish_date_picker(ui, id, var);
                                    ui.end_row();
                                };

                                text_row(ui, "Nombre:", &mut self.name);
                                text_row(ui, "RUT:", &mut self.rut);
                                
                                date_row(ui, "Fecha de Nacimiento:", "birth_picker", &mut self.birth);
                                date_row(ui, "Fecha Informe:", "today_picker", &mut self.today);
                                date_row(ui, "Fecha Cita:", "apmt_picker", &mut self.appointment);
                                
                                text_row(ui, "Hora Inicio:", &mut self.start_time);
                                text_row(ui, "Hora Fin:", &mut self.end_time);
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
    let d = date.day() as i32;
    let mut m = date.month() as i32;
    let y = date.year() as i32;

    let d_id = ui.make_persistent_id(format!("{}_d_text", id));
    let y_id = ui.make_persistent_id(format!("{}_y_text", id));

    let mut d_str = ui.data_mut(|di| di.get_temp::<String>(d_id).unwrap_or_else(|| d.to_string()));
    let mut y_str = ui.data_mut(|di| di.get_temp::<String>(y_id).unwrap_or_else(|| y.to_string()));

    let mut apply_change = |new_y: i32, new_m: i32, new_d: i32| {
        let clean_y = new_y.clamp(1, 9999) as i16;
        let clean_m = new_m.clamp(1, 12) as i8;
        let last = jiff::civil::Date::new(clean_y, clean_m, 1)
            .unwrap()
            .days_in_month();
        let clean_d = (new_d as i8).clamp(1, last as i8);

        if let Ok(nd) = jiff::civil::Date::new(clean_y, clean_m, clean_d) {
            *date = nd;
        }
    };

    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 8.0;

        let day_edit = ui.add(egui::TextEdit::singleline(&mut d_str).desired_width(33.0));
        if day_edit.changed() {
            if let Ok(val) = d_str.parse::<i32>() {
                apply_change(y, m, val);
            }
            ui.data_mut(|di| di.insert_temp(d_id, d_str.clone()));
        } else if !day_edit.has_focus() {
            d_str = d.to_string();
            ui.data_mut(|di| di.insert_temp(d_id, d_str.clone()));
        }

        let scroll_delta = ui.input(|i| {
            let mut delta = 0;
            for event in &i.raw.events {
                if let egui::Event::MouseWheel { delta: d, .. } = event {
                    if d.y > 0.0 {
                        delta = 1;
                    } else if d.y < 0.0 {
                        delta = -1;
                    }
                }
            }
            delta
        });

        let month_res = egui::ComboBox::from_id_salt(format!("{}_m", id))
            .selected_text(MESES[(m - 1) as usize])
            .width(150.0)
            .show_ui(ui, |ui| {
                for (i, nombre) in MESES.iter().enumerate() {
                    if ui
                        .selectable_value(&mut m, (i + 1) as i32, *nombre)
                        .changed()
                    {
                        apply_change(y, m, d);
                    }
                }
            })
            .response;

        if scroll_delta != 0 && month_res.hovered() {
            let mut new_m = m + scroll_delta;
            if new_m > 12 {
                new_m = 1;
            } else if new_m < 1 {
                new_m = 12;
            }
            apply_change(y, new_m, d);
            ui.input_mut(|i| i.smooth_scroll_delta = egui::Vec2::ZERO);
        }

        let year_edit = ui.add(egui::TextEdit::singleline(&mut y_str).desired_width(55.0));
        if year_edit.changed() {
            if let Ok(val) = y_str.parse::<i32>() {
                apply_change(val, m, d);
            }
            ui.data_mut(|di| di.insert_temp(y_id, y_str.clone()));
        } else if !year_edit.has_focus() {
            y_str = y.to_string();
            ui.data_mut(|di| di.insert_temp(y_id, y_str.clone()));
        }
    });
}
