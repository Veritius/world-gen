use bevy::prelude::*;
use bevy_egui::{egui::{menu, TopBottomPanel}, EguiContexts};
use crate::{state::SimulationState, people::{PersonBundle, personality::Personality}, common::DisplayName};
use super::editing::{BeingEdited, person::PersonListWindowOpen};

pub fn menu_bar_system(
    state: Res<State<SimulationState>>,
    mut ctxs: EguiContexts,
    mut commands: Commands,

    mut list_open: ResMut<PersonListWindowOpen>,
) {
    // Only show in setup
    if *state.get() != SimulationState::Setup { return; }

    // Add panel and menu bar
    TopBottomPanel::top("menubar_top_panel").show(ctxs.ctx_mut(), |ui| {
        menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Load from a file").clicked() {
                    
                }

                if ui.button("Save current state").clicked() {
                    
                }
            });

            ui.menu_button("People", |ui| {
                if ui.button("Show all people").clicked() {
                    list_open.0 = true;
                }

                if ui.button("Add person").clicked() {
                    commands.spawn((BeingEdited, PersonBundle {
                        name: DisplayName::new("Real McPerson"),
                        personality: Personality::default(),
                    }));
                }
            });

            ui.menu_button("Definitions", |ui| {

            });
        });
    });
}