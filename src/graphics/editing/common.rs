use bevy_egui::egui::{self, DragValue};
use crate::common::{DisplayName, Birthday, SimulationTime};
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

impl EguiEditableComponent for Birthday {
    type ReqData = SimulationTime;

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, time: Self::ReqData) {
        ui.horizontal(|ui| {
            ui.label("Birth date");
            ui.add(DragValue::new(&mut self.0)
            .clamp_range(0.0..=f32::INFINITY)
                .fixed_decimals(0)
                .custom_formatter(|n, _| {
                    let rnd = n as u64;
                    time.get_age_str(Birthday(rnd))
                })
            );
        });
    }
}