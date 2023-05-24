mod plots;

use std::sync::RwLockReadGuard;
use eframe::egui;
use crate::world::sim::SimulationBoundary;
use self::plots::{entity_count_plot, tick_time_plot};

use super::AppMemory;

pub(super) fn view_ui(
    ui: &mut egui::Ui,
    _memory: &mut AppMemory,
    sim: RwLockReadGuard<SimulationBoundary>,
) {
    let sim_ref = &*sim;
    let ctx = ui.ctx();

    // Start plots
    egui::Window::new("Tick time")
    .default_size([400.0, 120.0])
    .default_open(false)
    .show(ctx, |ui| {
        tick_time_plot(ui, sim_ref);
    });

    egui::Window::new("Entity count")
    .default_size([400.0, 120.0])
    .default_open(false)
    .show(ctx, |ui| {
        entity_count_plot(ui, sim_ref);
    });
    // End plots
}