use eframe::egui;
use jiff::civil::Date;

pub struct Calendar;

impl Calendar {
    const MONTHS: [&'static str; 12] = [
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

    pub fn show(ui: &mut egui::Ui, id: &str, date: &mut Date) {
        let d = date.day() as i32;
        let mut m = date.month() as i32;
        let y = date.year() as i32;

        let d_id = ui.make_persistent_id(format!("{}_d_text", id));
        let y_id = ui.make_persistent_id(format!("{}_y_text", id));

        let mut d_str =
            ui.data_mut(|di| di.get_temp::<String>(d_id).unwrap_or_else(|| d.to_string()));
        let mut y_str =
            ui.data_mut(|di| di.get_temp::<String>(y_id).unwrap_or_else(|| y.to_string()));

        let mut apply_change = |new_y: i32, new_m: i32, new_d: i32| {
            let clean_y = new_y.clamp(1, 9999) as i16;
            let clean_m = new_m.clamp(1, 12) as i8;
            let last = Date::new(clean_y, clean_m, 1).unwrap().days_in_month();
            let clean_d = (new_d as i8).clamp(1, last as i8);

            if let Ok(nd) = Date::new(clean_y, clean_m, clean_d) {
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
                .selected_text(Self::MONTHS[(m - 1) as usize])
                .width(150.0)
                .show_ui(ui, |ui| {
                    for (i, nombre) in Self::MONTHS.iter().enumerate() {
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
}
