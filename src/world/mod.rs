use rand::Rng;

pub mod event;
pub mod person;
pub mod place;
pub mod thing;

pub mod soft_limits {
    pub const MIN_YEARS_TO_SIMULATE: u32 = 50;
    pub const MAX_YEARS_TO_SIMULATE: u32 = 1000;
}

#[derive(Resource)]
pub struct WorldPregenConfig {
    pub name: String,
    pub random_seed: u32,
    pub history_starts_at: u32,
    pub years_to_simulate: u32,
    pub generation_direction: GenerationDirection,
    pub chaos_multiplier: f32,
}

impl Default for WorldPregenConfig {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            random_seed: rand::thread_rng().gen::<u32>(),
            history_starts_at: 0,
            generation_direction: GenerationDirection::Forwards,
            years_to_simulate: 100,
            chaos_multiplier: 1.0,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GenerationDirection {
    Forwards,
    Backwards,
}

use bevy_ecs::{prelude::Component, system::Resource};

/// The age of this entity.
#[derive(Component)]
pub struct EntityAge(u32);