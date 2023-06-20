//! The world map.
//! 
//! The map is a even-offset flat-top hexagonal grid based on https://www.redblobgames.com/grids/hexagons/

pub mod coordinates;
pub mod generation;
pub mod tile;

pub use coordinates::*;

use std::collections::BTreeMap;
use bevy::prelude::*;

#[derive(Resource)]
pub struct MapData {
    bounds: MapBounds,
    cells: BTreeMap<DoubledCoordinate, Entity>,
}

impl MapData {
    /// Returns the size of the map.
    pub fn bounds(&self) -> MapBounds {
        self.bounds
    }

    /// Returns a map of OffsetCoordinates to cell entities.
    pub fn cells(&self) -> &BTreeMap<DoubledCoordinate, Entity> {
        &self.cells
    }

    /// Returns a single map cell, if it exists.
    pub fn cell(&self, coord: MapCoordinate) -> Option<Entity> {
        let coord = coord.doubled();
        self.cells.get(&coord).cloned()
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MapBounds {
    min: IVec2,
    max: IVec2,
}

impl MapBounds {
    pub fn new(min: IVec2, max: IVec2) -> MapBounds {
        Self { min, max }
    }

    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }
}

#[derive(Component)]
pub struct MapCell {
    #[allow(dead_code)]
    pos: MapCoordinate,
}