use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::{params::{TimestepDirection, TimestepAmount}, time::{SimulationTime, SimulationInstant}};
use super::EguiEditable;

#[derive(Debug, Resource)]
pub struct SimulationSettingsWindowOpen(pub bool);

pub fn simulation_parameters_settings_window_system(
    mut ctxs: EguiContexts,
    mut window_open: ResMut<SimulationSettingsWindowOpen>,
    mut simulation_time: ResMut<SimulationTime>,
    mut timestep_dir: ResMut<TimestepDirection>,
    mut timestep_amt: ResMut<TimestepAmount>,
) {
    // Window is closed
    if !window_open.0 { return; }

    egui::Window::new("Simulation parameters")
    .show(ctxs.ctx_mut(), |ui| {
        egui::Grid::new("simulation_parameters_grid")
        .striped(true)
        .show(ui, |ui| {
            ui.label("Starting date");
            ui.horizontal(|ui| {
                simulation_time.start_day.show_edit_ui(ui, ());
                if ui.button("Reset").clicked() {
                    simulation_time.start_day = SimulationInstant::default();
                }
                simulation_time.current_day = simulation_time.start_day;
            });
            ui.end_row();

            ui.label("Timestep direction");
            timestep_dir.show_edit_ui(ui, "sim_params_time_direction");
            ui.end_row();

            ui.label("Timestep amount");
            timestep_amt.show_edit_ui(ui, "sim_params_time_amount");
            ui.end_row();
        });

        ui.separator();
        if ui.button("Finish").clicked() {
            window_open.0 = false;
        }
    });
}

impl EguiEditable for TimestepDirection {
    type ReqData = &'static str;

    fn show_edit_ui(&mut self, ui: &mut bevy_egui::egui::Ui, key: Self::ReqData) {
        egui::ComboBox::new(key, "")
        .selected_text(match self {
            Self::Forwards => "Forwards",
            Self::Backwards => "Backwards",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(self, Self::Forwards, "Forwards");
            ui.selectable_value(self, Self::Backwards, "Backwards");
        });
    }
}

impl EguiEditable for TimestepAmount {
    type ReqData = &'static str;

    fn show_edit_ui(&mut self, ui: &mut bevy_egui::egui::Ui, key: Self::ReqData) {
        egui::ComboBox::new(key, "")
        .selected_text(match self {
            Self::Days => "Days",
            Self::Months => "Months",
        })
        .show_ui(ui, |ui| {
            ui.selectable_value(self, Self::Days, "Days");
            ui.selectable_value(self, Self::Months, "Months");
        });
    }
}