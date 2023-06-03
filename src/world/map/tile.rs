use bevy::prelude::*;
use crate::world::common::Name;

#[derive(Bundle)]
pub struct MapTileDefBundle {
    pub name: Name,
    pub def: MapTileDefinition,
}

#[derive(Component)]
pub struct MapTileDefinition {
    pub accessed_by_land: bool,
    pub accessed_by_water: bool,

    pub movement_difficulty: f32,
    pub soil_fertility: f32,
}

impl Default for MapTileDefinition {
    fn default() -> Self {
        Self {
            accessed_by_land: true,
            accessed_by_water: false,
            
            movement_difficulty: 0.3,
            soil_fertility: 0.9,
        }
    }
}