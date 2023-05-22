//! An object in history.
//! Also non-specific data like the name or age of something.

use bevy::ecs::prelude::*;

/// Any entities with this component will have more in-depth information generated.
#[derive(Component, Clone)]
pub struct Important;

#[derive(Component, Clone)]
pub struct Name(pub String);

#[derive(Component, Clone)]
pub struct Age(pub u32);