//! An object in history.
//! Also non-specific data like the age of something.

use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Age(pub u32);