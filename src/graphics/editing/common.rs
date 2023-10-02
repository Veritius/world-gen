use bevy_egui::egui::{self, DragValue, emath::Numeric};
use crate::{common::DisplayName, time::{SimulationInstant, CreationDate}};
use super::EguiEditableComponent;

impl EguiEditableComponent for DisplayName {
    type ReqData = ();

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, _: Self::ReqData) {
        ui.horizontal(|ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut self.0);
        });
    }
}

impl EguiEditableComponent for CreationDate {
    type ReqData = SimulationInstant;

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, time: Self::ReqData) {
        ui.horizontal(|ui| {
            ui.label("Age");
            ui.add(DragValue::new(&mut self.0)
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