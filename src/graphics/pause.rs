use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::state::SimulationState;

/// Adds a UI to control the game state.
pub fn pause_menu_system(
    state: Res<State<SimulationState>>,
    mut next: ResMut<NextState<SimulationState>>,
    mut ctxs: EguiContexts,
) {
    // Only show while running
    if *state.get() == SimulationState::Setup { return; }

    // Create window
    egui::Window::new("simulation_control_panel")
    .title_bar(false)
    .resizable(false)
    .movable(false)
    .show(ctxs.ctx_mut(), |ui| {
        match state.get() {
            SimulationState::Paused => {
                if ui.button("Start").clicked() {
                    next.set(SimulationState::Running);
                }
            },
            SimulationState::Running => {
                if ui.button("Stop").clicked() {
                    next.set(SimulationState::Paused);
                }
            },
            _ => panic!()
        }
    });
}