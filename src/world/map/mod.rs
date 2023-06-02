//! The world map.
//! 
//! The map is a even-offset flat-top hexagonal grid based on https://www.redblobgames.com/grids/hexagons/

pub mod coordinates;

pub use coordinates::*;

use bevy::prelude::*;

#[derive(Component)]
pub struct MapCell {
    #[allow(dead_code)]
    pos: MapCoordinate,
}