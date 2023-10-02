use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, RichText}};
use crate::{common::{DisplayName, Age}, factions::Faction};
use super::BeingEdited;

#[derive(Debug, Resource)]
pub struct FactionListWindowOpen(pub bool);

/// Shows a window of every person
pub fn faction_listing_system(
    mut ctxs: EguiContexts,
    mut open: ResMut<FactionListWindowOpen>,
    mut commands: Commands,
    query: Query<(Entity, &DisplayName, &Age, Option<&BeingEdited>), With<Faction>>,
) {
    if !open.0 { return; }

    egui::Window::new("List of factions")
    .show(ctxs.ctx_mut(), |ui| {
        // Close window button
        if ui.button("Close window").clicked() {
            open.0 = false;
        }

        ui.separator();

        // Specify there is no people
        if query.is_empty() {
            ui.label(RichText::new("No factions to list").italics());
            return;
        }

        // List all people
        egui::ScrollArea::vertical()
        .show(ui, |ui| {
            egui::Grid::new("faction_list_grid")
            .striped(true)
            .show(ui, |ui| {
                for (entity, name, age, editing) in query.iter() {
                    ui.label(&name.0);
                    ui.label(format!("{}", age));
                    ui.horizontal(|ui| {
                        ui.add_enabled_ui(editing.is_none(), |ui| if ui.button("Edit").clicked() {
                            commands.entity(entity).insert(BeingEdited);
                        });
                        if ui.button("Delete").clicked() {
                            commands.entity(entity).despawn();
                        }
                    });
                    ui.end_row();
                }
            });
        });
    });
}