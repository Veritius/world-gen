use std::collections::BTreeMap;
use bevy::prelude::*;
use super::coordinates::HexCoordinate;

/// A group of map cells, for fast lookup and organisation.
#[derive(Debug, Default, Component)]
pub struct MapLayer {
    pub cells: BTreeMap<HexCoordinate, Entity>,
}

/// A single map cell in the world.
/// Always 'owned' by a [MapLayer].
#[derive(Debug, Component, Reflect)]
pub struct MapCell {
    pub owning_layer: Entity,
    
    pub terrain_elevation: f32,
    pub terrain_fertility: f32,
    pub terrain_rockiness: f32,
    pub movement_difficulty: f32,
}

/// Raised when a new map layer is first created.
#[derive(Debug, Event)]
pub struct MapLayerCreatedEvent(pub Entity);