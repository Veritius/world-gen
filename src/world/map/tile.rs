use bevy::prelude::*;
use crate::world::common::Name;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TerrainKind {
    Mountain,
    Normal,
    Water,
}

#[derive(Bundle)]
pub struct MapTileDefBundle {
    pub name: Name,
    pub def: MapTileDefinition,
}

#[derive(Component)]
pub struct MapTileDefinition {
    pub terrain_kind: TerrainKind,

    pub movement_difficulty: f32,
    pub soil_fertility: f32,
}

impl Default for MapTileDefinition {
    fn default() -> Self {
        Self {
            terrain_kind: TerrainKind::Normal,
            
            movement_difficulty: 0.3,
            soil_fertility: 0.9,
        }
    }
}