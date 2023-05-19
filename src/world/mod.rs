pub mod sim;
pub mod schedules;

pub mod event;
pub mod person;
pub mod place;
pub mod defs;
pub mod thing;

pub mod soft_limits {
    pub const MIN_YEARS_TO_SIMULATE: u32 = 50;
    pub const MAX_YEARS_TO_SIMULATE: u32 = 1000;
}

use bevy_ecs::prelude::Component;

/// The age of this entity.
#[derive(Component)]
pub struct EntityAge(u32);