use bevy::prelude::*;
use bevy_egui::{EguiContexts, egui::{self, Ui, RichText, Slider}};
use crate::{common::DisplayName, factions::{Faction, FACTION_INTEREST_RANGE}, time::{SimulationTime, CreationDate}};
use super::{BeingEdited, EguiEditableComponent};

#[derive(Debug, Resource)]
pub struct FactionListWindowOpen(pub bool);

impl EguiEditableComponent for Faction {
    type ReqData = Entity;

    fn show_edit_ui(&mut self, ui: &mut Ui, entity: Self::ReqData) {
        egui::Grid::new(format!("{:?}_fac_edt_interests", entity))
        .show(ui, |ui| {
            ui.label("Profit interest");
            ui.add(Slider::new(&mut self.profit_interest, FACTION_INTEREST_RANGE));
            ui.end_row();

            ui.label("Expansion interest");
            ui.add(Slider::new(&mut self.expansion_interest, FACTION_INTEREST_RANGE));
            ui.end_row();

            ui.label("Humanitarian interest");
            ui.add(Slider::new(&mut self.humanitarian_interest, FACTION_INTEREST_RANGE));
            ui.end_row();
        });
    }
}

/// Shows a window of every person
pub fn faction_listing_system(
    mut ctxs: EguiContexts,
    mut open: ResMut<FactionListWindowOpen>,
    mut commands: Commands,
    time: Res<SimulationTime>,
    query: Query<(Entity, &DisplayName, &CreationDate, Option<&BeingEdited>), With<Faction>>,
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
                for (entity, name, birthday, editing) in query.iter() {
                    ui.label(&name.0);
                    ui.label(format!("{}", birthday.0.since_saturating(time.current_day)));
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

/// Creates windows for editing factions
pub fn faction_editing_system(
    mut ctxs: EguiContexts,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Faction, &mut DisplayName, &mut CreationDate), With<BeingEdited>>,
    time: Res<SimulationTime>,
) {
    for (entity, mut faction, mut display_name, mut age) in query.iter_mut() {
        egui::Window::new(format!("{} ({:?})", display_name.0, entity))
        .show(ctxs.ctx_mut(), |ui| {
            display_name.show_edit_ui(ui, ());
            age.show_edit_ui(ui, time.current_day);

            ui.separator();

            faction.show_edit_ui(ui, entity);

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