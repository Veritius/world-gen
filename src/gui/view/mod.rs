use std::{sync::RwLockReadGuard, collections::BTreeMap};
use eframe::egui::Ui;
use crate::world::sim::SimulationBoundary;

pub(super) fn view_ui(
    ui: &mut Ui,
    state: &mut BTreeMap<String, String>,
    sim: RwLockReadGuard<SimulationBoundary>,
) {

}