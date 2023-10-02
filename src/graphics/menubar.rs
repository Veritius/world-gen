use bevy::prelude::*;
use bevy_egui::{egui::{menu, TopBottomPanel}, EguiContexts};
use crate::{state::SimulationState, people::{PersonBundle, personality::Personality, Person}, common::{DisplayName, Birthday}, factions::{FactionBundle, Faction}};
use super::editing::{BeingEdited, person::PersonListWindowOpen, factions::FactionListWindowOpen};

pub fn menu_bar_system(
    state: Res<State<SimulationState>>,
    mut ctxs: EguiContexts,
    mut commands: Commands,

    mut opened_lists: ParamSet<(
        ResMut<PersonListWindowOpen>,
        ResMut<FactionListWindowOpen>,
    )>,
) {
    // Only show in setup
    if *state.get() != SimulationState::Setup { return; }

    // Add panel and menu bar
    TopBottomPanel::top("menubar_top_panel").show(ctxs.ctx_mut(), |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Create new world").clicked() {
                    
                }

                if ui.button("Load from a file").clicked() {
                    
                }

                if ui.button("Save current state").clicked() {
                    
                }
            });

            ui.menu_button("People", |ui| {
                if ui.button("List people").clicked() {
                    opened_lists.p0().0 = true;
                }

                if ui.button("Add person").clicked() {
                    commands.spawn((BeingEdited, PersonBundle {
                        marker: Person,
                        name: DisplayName::new("Real McPerson"),
                        age: Birthday::from_years(18),
                        personality: Personality::default(),
                    }));
                }
            });

            ui.menu_button("Factions", |ui| {
                if ui.button("List factions").clicked() {
                    opened_lists.p1().0 = true;
                }

                if ui.button("Add faction").clicked() {
                    commands.spawn((BeingEdited, FactionBundle {
                        marker: Faction::default(),
                        name: DisplayName::new("A new faction"),
                        age: Birthday::from_days(0),
                    }));
                }
            });
        });
    });
}