mod meta;
mod people;
mod definitions;
mod places;
mod helpers;

use bevy::ecs::system::CommandQueue;
use eframe::egui;
use crate::world::sim::SimulationData;

use self::{
    meta::edit_meta_ui,
    people::edit_people_ui,
    definitions::edit_definitions_ui,
    places::edit_places_ui,
};

use super::AppMemory;

const TAB_KEY: &'static str = "edit_current_tab";

pub(super) fn edit_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    // Find the current tab
    if memory.string_map.get(TAB_KEY).is_none() { memory.string_map.insert(TAB_KEY.to_owned(), "Meta".to_string()); }
    let current_tab = memory.string_map.get_mut(TAB_KEY).unwrap();

    // Tab change buttons
    egui::ScrollArea::horizontal().auto_shrink([false, true]).show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_tab, "Meta".to_owned(), "Meta");
            ui.selectable_value(current_tab, "People".to_owned(), "People");
            ui.selectable_value(current_tab, "Definitions".to_owned(), "Definitions");
            ui.selectable_value(current_tab, "Places".to_owned(), "Places");
        });
    });

    ui.separator();

    // Tabs
    match current_tab.as_str() {
        "Meta" => edit_meta_ui(ui, memory, queue, sim),
        "People" => edit_people_ui(ui, memory, queue, sim),
        "Definitions" => edit_definitions_ui(ui, memory, queue, sim),
        "Places" => edit_places_ui(ui, memory, queue, sim),
        _ => todo!("Handle this case"),
    }
}