mod meta;
mod people;

use std::collections::BTreeMap;
use bevy_ecs::system::CommandQueue;
use eframe::egui;
use crate::world::sim::SimulationData;

use self::{
    meta::edit_meta_ui,
    people::edit_people_ui
};

const TAB_KEY: &'static str = "edit_current_tab";

pub(super) fn edit_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    // Find the current tab
    if state.get(TAB_KEY).is_none() { state.insert(TAB_KEY.to_owned(), "Meta".to_string()); }
    let current_tab = state.get_mut(TAB_KEY).unwrap();

    // Tab change buttons
    egui::ScrollArea::horizontal().auto_shrink([false, true]).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_tab, "Meta".to_owned(), "Meta");
            ui.selectable_value(current_tab, "People".to_owned(), "Historical Figures");
        });
    });

    ui.separator();

    // Tabs
    match current_tab.as_str() {
        "Meta" => edit_meta_ui(ui, state, queue, sim),
        "People" => edit_people_ui(ui, state, queue, sim),
        _ => todo!("Handle this case"),
    }
}