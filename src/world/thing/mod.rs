//! An object in history.
//! Also non-specific data like the name or age of something.

use bevy_ecs::prelude::*;

/// Any entities with this component will have more in-depth information generated.
#[derive(Component)]
pub struct Important;

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Age(pub u32);