use crate::errors::AssistanceGuiError;
use crate::utils::FileExplorer;
use crate::widgets::{Calendar, TimeRange as TimeWidget};
use rut::Rut;
use templates::Assistance;
use time_utils::TimeRange;

use eframe::egui;
use jiff::Zoned;
use jiff::civil::{Date, Time};

use std::sync::mpsc::{Receiver, Sender, channel};

pub struct AssistanceGui {
    name: Option<String>,
    birth: Date,
    rut: Option<Rut>,
    rut_buf: String,
    today: Date,
    appointment: Date,
    start_time: Time,
    end_time: Time,
    submitted: bool,
    is_saving: bool,
    tx: Sender<bool>,
    rx: Receiver<bool>,
}

impl Default for AssistanceGui {
    fn default() -> Self {
        let (tx, rx) = channel();
        let today = Zoned::now().date();

        Self {
            name: None,
            birth: today,
            rut_buf: String::new(),
            rut: None,
            today: today,
            appointment: today,
            start_time: Zoned::now().time(),
            end_time: Zoned::now().time(),
            submitted: false,
            is_saving: false,
            tx,
            rx,
        }
    }
}

impl eframe::App for AssistanceGui {
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
                                Self::text(ui, "Nombre:", &mut self.name, self.submitted);
                                Self::rut(
                                    ui,
                                    "RUT:",
                                    &mut self.rut_buf,
                                    &mut self.rut,
                                    self.submitted,
                                );

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

                if let Ok(finished) = self.rx.try_recv() {
                    self.is_saving = !finished;
                }

                ui.add_enabled_ui(!self.is_saving, |ui| {
                    let button = ui.add_sized([120.0, 40.0], egui::Button::new("Generar PDF"));

                    if button.clicked() {
                        self.submitted = true;

                        match Assistance::try_from(&*self) {
                            Ok(data) => {
                                self.is_saving = true;
                                let tx = self.tx.clone();

                                std::thread::spawn(move || {
                                    let name = format!(
                                        "certificado_asistencia_{}_{}",
                                        data.name().split(' ').next().unwrap_or("").to_lowercase(),
                                        data.today()
                                    );
                                    let _ = FileExplorer::save(&name, &data);
                                    let _ = tx.send(true);
                                });
                            }
                            Err(_) => {}
                        }
                    }
                });
            });
        });
    }
}

impl TryFrom<&AssistanceGui> for Assistance {
    type Error = AssistanceGuiError;

    fn try_from(gui: &AssistanceGui) -> Result<Self, Self::Error> {
        let range = TimeRange::try_new(gui.start_time, gui.end_time)?;
        let name = gui.name.as_ref().ok_or(AssistanceGuiError::EmptyName)?;
        let rut = gui.rut.clone().ok_or(AssistanceGuiError::EmptyName)?;

        Ok(Assistance::try_new(
            name,
            gui.birth,
            rut,
            gui.today,
            gui.appointment,
            range,
        )?)
    }
}

impl AssistanceGui {
    fn text(ui: &mut egui::Ui, lbl: &str, var: &mut Option<String>, submitted: bool) {
        ui.label(lbl);
        let id = ui.make_persistent_id(lbl);

        let mut tmp_name = var.as_deref().unwrap_or("").to_string();

        ui.scope(|ui| {
            if Self::should_show_error(ui, id, submitted, tmp_name.is_empty()) {
                Self::apply_error_style(ui);
            }

            let res =
                ui.add(egui::TextEdit::singleline(&mut tmp_name).desired_width(f32::INFINITY));

            if res.changed() {
                *var = (!tmp_name.is_empty()).then_some(tmp_name);
                ui.data_mut(|d| d.insert_temp(id, true));
            }
            if res.gained_focus() {
                ui.data_mut(|d| d.insert_temp(id, true));
            }
        });

        ui.end_row();
    }

    fn date(ui: &mut egui::Ui, lbl: &str, id: &str, var: &mut Date) {
        ui.label(lbl);
        Calendar::show(ui, id, var);
        ui.end_row();
    }

    fn time(ui: &mut egui::Ui, lbl: &str, id: &str, start: &mut Time, end: &mut Time) {
        ui.label(lbl);
        TimeWidget::show(ui, id, start, end);
        ui.end_row();
    }

    fn rut(
        ui: &mut egui::Ui,
        lbl: &str,
        buffer: &mut String,
        rut_val: &mut Option<Rut>,
        submitted: bool,
    ) {
        ui.label(lbl);
        let id = ui.make_persistent_id(lbl);

        ui.scope(|ui| {
            let is_invalid = buffer.is_empty() || buffer.parse::<Rut>().is_err();

            if Self::should_show_error(ui, id, submitted, is_invalid) {
                Self::apply_error_style(ui);
            }

            let res = ui.add(egui::TextEdit::singleline(buffer).desired_width(f32::INFINITY));

            if res.changed() {
                *rut_val = buffer.parse::<Rut>().ok();
                ui.data_mut(|d| d.insert_temp(id, true));
            }

            if res.gained_focus() {
                ui.data_mut(|d| d.insert_temp(id, true));
            }

            if res.lost_focus() {
                if let Some(valid) = rut_val {
                    *buffer = valid.to_string();
                }
            }
        });

        ui.end_row();
    }

    fn should_show_error(ui: &egui::Ui, id: egui::Id, submitted: bool, is_invalid: bool) -> bool {
        let interacted = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
        is_invalid && (interacted || submitted)
    }

    fn apply_error_style(ui: &mut egui::Ui) {
        let visuals = &mut ui.style_mut().visuals.widgets;
        let stroke = egui::Stroke::new(1.0, egui::Color32::RED);
        visuals.inactive.bg_stroke = stroke;
        visuals.hovered.bg_stroke = stroke;
        visuals.active.bg_stroke = stroke;
    }
}
