use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, RichText, Ui}};
use crate::{species::Species, common::DisplayName};
use super::{BeingEdited, EguiEditable};

#[derive(Debug, Resource)]
pub struct SpeciesListWindowOpen(pub bool);

/// Shows a window of every species
pub fn species_listing_system(
    mut ctxs: EguiContexts,
    mut open: ResMut<SpeciesListWindowOpen>,
    mut commands: Commands,
    query: Query<(Entity, &Species, &DisplayName, Option<&BeingEdited>)>,
) {
    if !open.0 { return; }

    egui::Window::new("List of species")
    .show(ctxs.ctx_mut(), |ui| {
        // Close window button
        if ui.button("Close window").clicked() {
            open.0 = false;
        }

        ui.separator();

        // Specify there is no people
        if query.is_empty() {
            ui.label(RichText::new("No species to list").italics());
            return;
        }

        // Count amounts to hide unnecessary UI
        let mut humanoids: u32 = 0;
        let mut animals: u32 = 0;
        for (_, species, _, _) in query.iter() {
            if species.is_humanoid {
                humanoids += 1;
            } else {
                animals += 1;
            }
        }

        // Separate species into humanoids and animals
        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                if humanoids != 0 {
                    ui.collapsing("Humanoids", |ui| {
                        egui::Grid::new("humanoids_list_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            for (entity, species, display_name, being_edited) in query.iter() {
                                if !species.is_humanoid { continue; }
                                add_entry_to_species_list(
                                    ui,
                                    &mut commands,
                                    entity,
                                    // species,
                                    display_name,
                                    being_edited.is_some()
                                );
                            }
                        });
                    });
                }
        
                if animals != 0 {
                    ui.collapsing("Animals", |ui| {
                        egui::Grid::new("animals_list_grid")
                        .striped(true)
                        .show(ui, |ui| {
                            for (entity, species, display_name, being_edited) in query.iter() {
                                if species.is_humanoid { continue; }
                                add_entry_to_species_list(
                                    ui,
                                    &mut commands,
                                    entity,
                                    // species,
                                    display_name,
                                    being_edited.is_some()
                                );
                            }
                        });
                    });
                }
            });
    });
}

fn add_entry_to_species_list(
    ui: &mut Ui,
    commands: &mut Commands,
    entity: Entity,
    // species: &Species,
    display_name: &DisplayName,
    being_edited: bool,
) {
    ui.label(&display_name.0);
    ui.horizontal(|ui| {
        ui.add_enabled_ui(!being_edited, |ui| {
            if ui.button("Edit").clicked() {
                commands.entity(entity).insert(BeingEdited);
            }
        });
        if ui.button("Delete").clicked() {
            commands.entity(entity).despawn();
        }
    });
    ui.end_row();
}

/// Creates windows for editing species
pub fn species_editing_system(
    mut ctxs: EguiContexts,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Species, &mut DisplayName), With<BeingEdited>>,
) {
    for (entity, mut species, mut display_name) in query.iter_mut() {
        egui::Window::new(format!("{} ({:?})", display_name.0, entity))
        .show(ctxs.ctx_mut(), |ui| {
            // Main controls
            egui::Grid::new(format!("species_{:?}_edit_grid", entity))
            .striped(true)
            .show(ui, |ui| {
                ui.label("Species name");
                display_name.show_edit_ui(ui, ());
                ui.end_row();

                ui.label("Humanoid");
                ui.checkbox(&mut species.is_humanoid, "");
                ui.end_row();

                ui.label("Age of maturity");
                species.age_of_maturity.show_edit_ui(ui, true);
            });

            ui.separator();

            // Exit buttons
            ui.horizontal(|ui| {
                if ui.button("Finish").clicked() {
                    // Remove editing indicator to hide window
                    commands.entity(entity).remove::<BeingEdited>();
                }
                if ui.button("Save").clicked() {
                    // Save to disk
                    todo!()
                }
                if ui.button("Delete").clicked() {
                    // Despawn the entity
                    commands.entity(entity).despawn();
                }
            });
        });
    }
}