pub mod params;
pub mod common;
pub mod time;
pub mod person;
pub mod factions;
pub mod species;

use bevy::prelude::*;

/// Flags an entity as currently being edited in the UI.
#[derive(Debug, Component, Reflect)]
pub struct BeingEdited;

/// A component that can be shown in egui. Different to reflection-powered access, this has a manually written UI.
pub trait EguiEditable {
    type ReqData;

    fn show_edit_ui(&mut self, ui: &mut bevy_egui::egui::Ui, req: Self::ReqData);
}