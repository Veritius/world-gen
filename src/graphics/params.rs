use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{params::{TimestepDirection, TimestepAmount}, time::SimulationTime};

#[derive(Debug, Resource)]
pub struct SimulationSettingsWindowOpen(pub bool);

pub fn simulation_parameters_settings_window_system(
    mut ctxs: EguiContexts,
    mut window_open: ResMut<SimulationSettingsWindowOpen>,
    mut simulation_time: ResMut<SimulationTime>,
    mut timestep_dir: ParamSet<(Res<State<TimestepDirection>>, ResMut<NextState<TimestepDirection>>)>,
    mut timestep_amt: ParamSet<(Res<State<TimestepAmount>>, ResMut<NextState<TimestepAmount>>)>,
) {
    // Window is closed
    if !window_open.0 { return; }

    egui::Window::new("Simulation parameters")
    .show(ctxs.ctx_mut(), |ui| {
        
    });
}