use eframe::egui;
use crate::world::WorldPregenConfig;

#[derive(Default)]
pub struct ConfigState {
    tab: Tab,
    world: WorldPregenConfig,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tab {
    #[default]
    Meta,
}

pub fn config_ui(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab, Tab::Meta, "Meta");
    });

    ui.group(|ui| {
        match &state.tab {
            Tab::Meta => tab_meta(ui, state),
        }
    });
}

fn tab_meta(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.add(egui::TextEdit::singleline(&mut state.world.name).hint_text("World name"));
}