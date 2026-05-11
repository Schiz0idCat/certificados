use eframe::egui;
use jiff::civil::Time;

pub struct TimeRange;

impl TimeRange {
    pub fn show(ui: &mut egui::Ui, id: &str, start: &mut Time, end: &mut Time) {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 8.0;

            ui.label("Desde:");
            Self::render_time_cells(ui, format!("{}_s", id), start);

            ui.add_space(10.0);

            ui.label("Hasta:");
            Self::render_time_cells(ui, format!("{}_e", id), end);
        });

        // Lógica de restricción: Si 'end' es menor que 'start', igualar.
        if *end < *start {
            *end = *start;

            // Forzamos la actualización de los strings temporales en el siguiente frame
            // al limpiar los datos cacheados de las celdas de "Hasta"
            let id_e = format!("{}_e", id);
            ui.data_mut(|di| {
                di.remove_temp::<String>(ui.make_persistent_id(format!("{}_h", id_e)));
                di.remove_temp::<String>(ui.make_persistent_id(format!("{}_m", id_e)));
            });
        }
    }

    fn render_time_cells(ui: &mut egui::Ui, id: String, time: &mut Time) {
        let h_id = ui.make_persistent_id(format!("{}_h", id));
        let m_id = ui.make_persistent_id(format!("{}_m", id));

        let mut h_str = ui.data_mut(|di| {
            di.get_temp::<String>(h_id)
                .unwrap_or_else(|| format!("{:02}", time.hour()))
        });
        let mut m_str = ui.data_mut(|di| {
            di.get_temp::<String>(m_id)
                .unwrap_or_else(|| format!("{:02}", time.minute()))
        });

        ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let res_h = ui.add(egui::TextEdit::singleline(&mut h_str).desired_width(33.0));
            if res_h.changed() {
                if let Ok(v) = h_str.parse::<i32>() {
                    if let Ok(nt) = Time::new(v.clamp(0, 23) as i8, time.minute() as i8, 0, 0) {
                        *time = nt;
                    }
                }
                ui.data_mut(|di| di.insert_temp(h_id, h_str.clone()));
            } else if !res_h.has_focus() {
                h_str = format!("{:02}", time.hour());
                ui.data_mut(|di| di.insert_temp(h_id, h_str.clone()));
            }

            ui.label(":");

            let res_m = ui.add(egui::TextEdit::singleline(&mut m_str).desired_width(32.0));
            if res_m.changed() {
                if let Ok(v) = m_str.parse::<i32>() {
                    if let Ok(nt) = Time::new(time.hour() as i8, v.clamp(0, 59) as i8, 0, 0) {
                        *time = nt;
                    }
                }
                ui.data_mut(|di| di.insert_temp(m_id, m_str.clone()));
            } else if !res_m.has_focus() {
                m_str = format!("{:02}", time.minute());
                ui.data_mut(|di| di.insert_temp(m_id, m_str.clone()));
            }
        });
    }
}
