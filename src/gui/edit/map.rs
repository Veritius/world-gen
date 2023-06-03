use bevy::ecs::system::CommandQueue;
use eframe::egui;
use crate::{gui::AppMemory, world::sim::SimulationData};

pub(super) fn edit_map_ui(
    ui: &mut egui::Ui,
    _memory: &mut AppMemory,
    _queue: &mut CommandQueue,
    _sim: &mut SimulationData,
) {
    ui.heading("Work in progress!");
}