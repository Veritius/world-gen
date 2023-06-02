pub mod sim;
pub mod presets;
pub mod time;

pub mod common;
pub mod event;
pub mod person;
pub mod place;
pub mod defs;
pub mod living;
pub mod faction;
pub mod map;

pub mod soft_limits {
    pub const MIN_YEARS_TO_SIMULATE: u32 = 50;
    pub const MAX_YEARS_TO_SIMULATE: u32 = 1000;
}

use bevy::prelude::Component;

/// The age of this entity.
#[derive(Component)]
pub struct EntityAge(u32);