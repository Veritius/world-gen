use bevy::{ecs::system::{CommandQueue, SystemState}, prelude::{Query, Entity}};
use eframe::egui::{self, plot::{Polygon, PlotPoints}};
use crate::{gui::AppMemory, world::{sim::SimulationData, map::{tile::MapTileDefinition, MapCell}}};

pub(super) fn edit_map_ui(
    ui: &mut egui::Ui,
    _memory: &mut AppMemory,
    _queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    map_window(ui, sim);
}

pub(super) fn map_window(
    ui: &mut egui::Ui,
    sim: &mut SimulationData,
) {
    egui::plot::Plot::new("map_editor_canvas")
    .data_aspect(1.0)
    .show_axes([false, false])
    .show(ui, |plot| {
        let mut state: SystemState<(
            Query<(Entity, &MapTileDefinition)>,
            Query<&MapCell>,
        )> = SystemState::new(&mut sim.app.world);

        let state = state.get(&sim.app.world);
        let mapdefs = state.0;

        for cell in state.1.iter() {
            let pos = cell.pos.axial().cartesian().as_dvec2();
            let points = PlotPoints::from_iter([
                [pos.x + 0.5, pos.y + 0.866],
                [pos.x + -0.5, pos.y + 0.866],
                [pos.x + -1.0, pos.y + 0.0],
                [pos.x + -0.5, pos.y + -0.866],
                [pos.x + 0.5, pos.y + -0.866],
                [pos.x + 1.0, pos.y + 0.0],
            ]);
            let poly = Polygon::new(points).highlight(true);
            plot.polygon(poly);
        }
    });
}