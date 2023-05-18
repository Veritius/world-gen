use std::collections::BTreeMap;
use bevy_ecs::system::CommandQueue;
use eframe::egui;
use crate::world::sim::SimulationData;

pub(super) fn edit_meta_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {

}