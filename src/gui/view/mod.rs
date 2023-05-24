mod plots;

use std::sync::RwLockReadGuard;
use eframe::egui;
use crate::world::sim::SimulationBoundary;
use self::plots::stat_plots;

use super::AppMemory;

pub(super) fn view_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    sim: RwLockReadGuard<SimulationBoundary>,
) {
    ui.horizontal(|ui| {
        if ui.button("Stop simulation").clicked() {
            memory.markers.insert("try_freeze_simulation".to_owned());
        }

        let percent = sim.steps_complete as f32 / sim.steps_total as f32;
        ui.add(egui::ProgressBar::new(percent).show_percentage());
    });

    ui.separator();

    stat_plots(ui, sim);
}