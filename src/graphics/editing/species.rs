use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, RichText}};
use crate::{species::Species, common::DisplayName};
use super::BeingEdited;

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
    });
}