use bevy_egui::egui;
use crate::common::DisplayName;
use super::EguiEditable;

impl EguiEditable for DisplayName {
    type ReqData = ();

    fn show_edit_ui(&mut self, ui: &mut egui::Ui, _: Self::ReqData) {
        ui.text_edit_singleline(&mut self.0);
    }
}