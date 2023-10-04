//! The world map.

pub mod coordinates;
pub mod generation;
pub mod cells;

use bevy::prelude::*;
use self::generation::*;

/// Config for the simulation map generation.
#[derive(Debug, Default, Resource)]
pub struct SimulationMap {
    pub random_seed: u64,
    pub gen_method: WorldGenerationMethod,
    pub map_size: UVec2,
}

pub fn add_map_code_to_app(app: &mut App) {
    app.add_event::<ClearAndRegenerateMapEvent>();
    app.init_resource::<SimulationMap>();
}