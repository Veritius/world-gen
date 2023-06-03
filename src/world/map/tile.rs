use bevy::prelude::*;
use crate::world::common::Name;

#[derive(Bundle)]
pub struct MapTileDefBundle {
    pub name: Name,
    pub def: MapTileDefinition,
}

#[derive(Component)]
pub struct MapTileDefinition {
    pub passable_by_land: bool,
    pub passable_by_water: bool,
    pub passable_by_air: bool,
}

impl Default for MapTileDefinition {
    fn default() -> Self {
        Self {
            passable_by_land: true,
            passable_by_water: false,
            passable_by_air: true,
        }
    }
}