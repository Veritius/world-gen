use bevy::prelude::*;
use bevy_egui::{egui::{self, Ui, RichText, Slider}, EguiContexts};
use crate::{common::{DisplayName, Birthday, SimulationTime}, people::{personality::{Personality, PERSONALITY_VALUE_RANGE}, Person}};
use super::{BeingEdited, EguiEditableComponent};

#[derive(Debug, Resource)]
pub struct PersonListWindowOpen(pub bool);

impl EguiEditableComponent for Personality {
    type ReqData = Entity;

    fn show_edit_ui(&mut self, ui: &mut Ui, entity: Self::ReqData) {
        egui::Grid::new(format!("{:?}_personality_editor", entity))
        .show(ui, |ui| {
            ui.label("Selflessness");
            ui.add(Slider::new(&mut self.selflessness, PERSONALITY_VALUE_RANGE));
            ui.end_row();

            ui.label("Aggression");
            ui.add(Slider::new(&mut self.aggression, PERSONALITY_VALUE_RANGE));
            ui.end_row();
        });
    }
}

/// Shows a window of every person
pub fn person_listing_system(
    mut ctxs: EguiContexts,
    mut open: ResMut<PersonListWindowOpen>,
    mut commands: Commands,
    time: Res<SimulationTime>,
    query: Query<(Entity, &DisplayName, &Birthday, Option<&BeingEdited>), With<Person>>,
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
        egui::ScrollArea::vertical()
        .show(ui, |ui| {
            egui::Grid::new("people_list_grid")
            .striped(true)
            .show(ui, |ui| {
                for (entity, name, birthday, editing) in query.iter() {
                    ui.label(&name.0);
                    ui.label(format!("{}", time.get_age_str(*birthday)));
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

/// Creates windows for editing people
pub fn person_editing_system(
    mut ctxs: EguiContexts,
    mut commands: Commands,
    mut query: Query<(Entity, &mut DisplayName, &mut Birthday, &mut Personality), (With<Person>, With<BeingEdited>)>,
    time: Res<SimulationTime>,
) {
    for (entity, mut display_name, mut age, mut personality) in query.iter_mut() {
        egui::Window::new(format!("{} ({:?})", display_name.0, entity))
        .show(ctxs.ctx_mut(), |ui| {
            display_name.show_edit_ui(ui, ());
            age.show_edit_ui(ui, *time);

            ui.separator();

            personality.show_edit_ui(ui, entity);

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