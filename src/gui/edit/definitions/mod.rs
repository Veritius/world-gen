mod afflictions;
mod species;
mod tile;

use bevy::ecs::system::CommandQueue;
use eframe::egui;
use crate::{world::sim::SimulationData, gui::AppMemory};
use afflictions::afflictions_menu;
use species::species_menu;

use self::tile::tiles_menu;

const SUBTAB_KEY: &str = "edit_definitions_tab";

pub(super) fn edit_definitions_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    if memory.string_map.get(SUBTAB_KEY).is_none() { memory.string_map.insert(SUBTAB_KEY.to_owned(), "".to_string()); }
    let current_tab = memory.string_map.get_mut(SUBTAB_KEY).unwrap();

    // Tab change buttons
    egui::ScrollArea::horizontal()
    .id_source("defs_scroll_area")
    .auto_shrink([false, true])
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_tab, "Afflictions".to_owned(), "Afflictions");
            ui.selectable_value(current_tab, "Species".to_owned(), "Species");
            ui.selectable_value(current_tab, "Tiles".to_owned(), "Map tiles");
        });
    });
    
    ui.separator();
    
    // Tabs
    match current_tab.as_str() {
        "Afflictions" => afflictions_menu(ui, queue, sim),
        "Species" => species_menu(ui, queue, sim),
        "Tiles" => tiles_menu(ui, queue, sim),
        _ => {},
    }
}