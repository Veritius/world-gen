use bevy::prelude::*;

#[derive(Debug, Component, Reflect)]
pub struct MapCell {
    pub terrain_elevation: f32,
    pub terrain_fertility: f32,
    pub terrain_rockiness: f32,
    pub movement_difficulty: f32,
}

#[derive(Debug, Event)]
pub struct MapCellCreatedEvent {
    pub cell_coordinate: (),
    pub cell_identifier: Entity,
}
