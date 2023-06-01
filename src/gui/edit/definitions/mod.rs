mod species;

use bevy::ecs::system::CommandQueue;
use eframe::egui;
use crate::{world::sim::SimulationData, gui::AppMemory};
use species::species_menu;

const SUBTAB_KEY: &str = "edit_definitions_tab";

pub(super) fn edit_definitions_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    if memory.string_map.get(SUBTAB_KEY).is_none() { memory.string_map.insert(SUBTAB_KEY.to_owned(), "Species".to_string()); }
    let current_tab = memory.string_map.get_mut(SUBTAB_KEY).unwrap();

    // Tab change buttons
    egui::ScrollArea::horizontal()
    .id_source("defs_scroll_area")
    .auto_shrink([false, true])
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_tab, "Species".to_owned(), "Species");
        });
    });
    
    ui.separator();
    
    // Tabs
    match current_tab.as_str() {
        "Species" => species_menu(ui, queue, sim),
        _ => todo!("Handle this case"),
    }
}