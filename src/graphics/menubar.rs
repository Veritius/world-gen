use bevy::prelude::*;
use bevy_egui::{egui::{menu, TopBottomPanel}, EguiContexts};
use crate::{params::SimulationState, people::{PersonBundle, personality::Personality, Person}, common::DisplayName, factions::{FactionBundle, Faction}, time::{SimulationTime, SimulationDuration, CreationDate}};
use super::editing::{BeingEdited, person::PersonListWindowOpen, factions::FactionListWindowOpen, params::SimulationSettingsWindowOpen};

pub fn menu_bar_system(
    state: Res<State<SimulationState>>,
    mut ctxs: EguiContexts,
    mut commands: Commands,
    time: Res<SimulationTime>,

    mut opened_windows: ParamSet<(
        ResMut<PersonListWindowOpen>,
        ResMut<FactionListWindowOpen>,
        ResMut<SimulationSettingsWindowOpen>,
    )>,
) {
    // Only show in setup
    if *state.get() != SimulationState::Setup { return; }

    // Add panel and menu bar
    TopBottomPanel::top("menubar_top_panel").show(ctxs.ctx_mut(), |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("Simulation", |ui| {
                if ui.button("Create new world").clicked() {
                    
                }

                if ui.button("Load from a file").clicked() {
                    
                }

                if ui.button("Save current state").clicked() {
                    
                }

                if ui.button("Change parameters").clicked() {
                    opened_windows.p2().0 = true;
                }
            });

            ui.menu_button("People", |ui| {
                if ui.button("List people").clicked() {
                    opened_windows.p0().0 = true;
                }

                if ui.button("Add person").clicked() {
                    commands.spawn((BeingEdited, PersonBundle {
                        marker: Person,
                        name: DisplayName::new("Real McPerson"),
                        age: CreationDate(time.current_day - SimulationDuration::years(18)),
                        personality: Personality::default(),
                    }));
                }
            });

            ui.menu_button("Factions", |ui| {
                if ui.button("List factions").clicked() {
                    opened_windows.p1().0 = true;
                }

                if ui.button("Add faction").clicked() {
                    commands.spawn((BeingEdited, FactionBundle {
                        marker: Faction::default(),
                        name: DisplayName::new("A new faction"),
                        age: CreationDate(time.current_day - SimulationDuration::years(18)),
                    }));
                }
            });
        });
    });
}