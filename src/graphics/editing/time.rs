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
    type ReqData = ();

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, _: Self::ReqData) {
        ui.add(egui::DragValue::new(self)
            .custom_formatter(|n, _| {
                format!("{}", SimulationDuration::from_f64(n))
            })
            .custom_parser(|_s| {
                todo!()
            }));
    }
}

impl EguiEditable for CreationDate {
    type ReqData = SimulationInstant;

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, time: Self::ReqData) {
        ui.horizontal(|ui| {
            ui.label("Age");
            ui.add(egui::DragValue::new(&mut self.0)
            .clamp_range(0.0..=f32::INFINITY)
                .fixed_decimals(0)
                .custom_formatter(|n, _| {
                    let instant = SimulationInstant::from_f64(n);
                    format!("{}", instant.since_saturating(time))
                })
            );
        });
    }
}