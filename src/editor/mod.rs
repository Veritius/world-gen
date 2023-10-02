use bevy::prelude::*;

/// Flags an entity as currently being edited.
#[derive(Debug, Component, Reflect)]
pub struct BeingEdited;