use bevy::prelude::*;
use bevy_egui::{egui::{self, Color32, RichText}, EguiContexts};
use crate::{common::DisplayName, people::personality::{Personality, PERSONALITY_VALUE_RANGE}};
use super::{BeingEdited, helpers::{titled_slider, titled_text}};

#[derive(Debug, Resource)]
pub struct PersonListWindowOpen(pub bool);

/// Shows a window of every person
pub fn person_listing_system(
    mut ctxs: EguiContexts,
    mut open: ResMut<PersonListWindowOpen>,
    mut commands: Commands,
    query: Query<(Entity, &DisplayName, Option<&BeingEdited>)>,
) {
    if !open.0 { return; }

    egui::Window::new("List of people")
    .show(ctxs.ctx_mut(), |ui| {
        // Close window button
        if ui.button("Close window").clicked() {
            open.0 = false;
        }

        ui.separator();

        // Specify there is no people
        if query.is_empty() {
            ui.label(RichText::new("Nobody to list").italics());
            return;
        }

        // List all people
        ui.vertical(|ui| {
            for (entity, name, editing) in query.iter() {
                if ui.button(&name.0).clicked() {
                    commands.entity(entity).insert(BeingEdited);
                }
            }
        });
    });
}

/// Creates windows for editing people
pub fn person_editing_system(
    mut ctxs: EguiContexts,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DisplayName, &mut Personality), With<BeingEdited>>,
) {
    for (entity, mut display_name, mut personality) in query.iter_mut() {
        egui::Window::new(format!("{} ({:?})", display_name.0, entity))
        .show(ctxs.ctx_mut(), |ui| {
            titled_text(ui, "Name", &mut display_name.0);

            ui.separator();
            titled_slider(ui, "Selflessness", &mut personality.selflessness, PERSONALITY_VALUE_RANGE);
            titled_slider(ui, "Aggression", &mut personality.aggression, PERSONALITY_VALUE_RANGE);

            ui.separator();
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