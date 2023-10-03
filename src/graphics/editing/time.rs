use bevy_egui::egui::{self, emath::Numeric};
use crate::time::{SimulationInstant, SimulationDuration, CreationDate};
use super::EguiEditable;

impl EguiEditable for SimulationInstant {
    type ReqData = ();

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, _: Self::ReqData) {
        ui.add(egui::DragValue::new(self)
            .custom_formatter(|n, _| {
                format!("{}", SimulationInstant::from_f64(n))
            })
            .custom_parser(|_s| {
                todo!()
            }));
    }
}

impl EguiEditable for SimulationDuration {
    type ReqData = bool;

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, split: Self::ReqData) {
        if split {
            ui.add(egui::DragValue::new(&mut self.0)
                .speed(365)
                .custom_formatter(|n, _| {
                    let v = n as u64 / 365;
                    format!("{v} years")
                }));
            ui.add(egui::DragValue::new(&mut self.0)
                .speed(1)
                .custom_formatter(|n, _| {
                    let v = n as u64 % 365;
                    format!("{v} days")
                }));
        } else {
            ui.add(egui::DragValue::new(self)
                .custom_formatter(|n, _| {
                    format!("{}", SimulationDuration::from_f64(n))
                })
                .custom_parser(|_s| {
                    todo!()
                }));
        }
    }
}

impl EguiEditable for CreationDate {
    type ReqData = SimulationInstant;

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, time: Self::ReqData) {
        ui.horizontal(|ui| {

            ui.label("Age");
            ui.add(egui::DragValue::new(&mut self.0)
                .clamp_range(time.to_f64() as f32..=f32::INFINITY)
                .fixed_decimals(0)
                .custom_formatter(|n, _| {
                    let instant = SimulationInstant::from_f64(n);
                    match instant.since(time) {
                        Some(val) => format!("{}", val),
                        None => format!("Older than time!"),
                    }
                })
            );
        });
    }
}