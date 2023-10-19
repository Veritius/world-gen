//! The world map.

pub mod coordinates;
pub mod generation;
pub mod cells;

use bevy::prelude::*;
use self::generation::{*, initial::generation_dispatch_system};

/// Config for the simulation map generation.
#[derive(Debug, Resource)]
pub struct SimulationMap {
    pub random_seed: u32,
    pub gen_method: WorldGenerationMethod,
    pub map_size: UVec2,
}

impl Default for SimulationMap {
    fn default() -> Self {
        Self {
            random_seed: fastrand::u32(0..=u32::MAX),
            gen_method: Default::default(),
            map_size: UVec2::splat(1),
        }
    }
}

pub fn add_map_code_to_app(app: &mut App) {
    app.add_event::<RegenerateMapEvent>();
    app.init_resource::<SimulationMap>();

    app.add_systems(Update, generation_dispatch_system);
}