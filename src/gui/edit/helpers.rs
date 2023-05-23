//! Generic helper functions and widgets for various editor UIs.

use bevy::{prelude::*, ecs::system::CommandQueue};
use eframe::egui;
use crate::gui::EntityStringHashable;

/// Creates a button that allows changing an entity's parent.
pub fn change_owner_button(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    options: &Vec<(Entity, String)>,
    entity: Entity,
) {
    egui::ComboBox::from_id_source(EntityStringHashable(entity, "change_owner_button".to_string()))
    .selected_text("Change parent")
    .width(150.0)
    .show_ui(ui, |ui| {
        if ui.button("Remove owner").clicked() {
            queue.push(RemoveParent { child: entity });
        };

        // List all regions
        for (region_ent, region_name) in options {
            if *region_ent == entity { continue; } // don't set ourselves as the parent
            if ui.button(format!("{} ({:?})", region_name, region_ent)).clicked() {
                queue.push(AddChild { parent: *region_ent, child: entity });
            }
        }
    });
}