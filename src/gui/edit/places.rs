use std::collections::BTreeMap;
use bevy::ecs::system::CommandQueue;
use eframe::egui;
use crate::world::sim::SimulationData;

pub(super) fn edit_places_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {

}