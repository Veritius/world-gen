pub mod person;

mod helpers;

use bevy::prelude::*;

/// Flags an entity as currently being edited in the UI.
#[derive(Debug, Component, Reflect)]
pub struct BeingEdited;