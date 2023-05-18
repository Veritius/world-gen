use std::sync::RwLockReadGuard;
use eframe::egui::Ui;
use crate::world::sim::SimulationBoundary;

pub(super) fn view_ui(
    ui: &mut Ui,
    sim: RwLockReadGuard<SimulationBoundary>,
) {

}