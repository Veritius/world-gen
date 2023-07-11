use std::ops::{Add, Sub, AddAssign, SubAssign};

use bevy::prelude::Vec2;

/// A hex map coordinate that can be in cube or offset form.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapCoordinate {
    /// Cube hex coordinates.
    Axial(AxialCoordinate),
    /// Offset hex coordinates (doubled!)
    Doubled(DoubledCoordinate),
}

impl MapCoordinate {
    /// Returns an `AxialCoordinate`, converting from `DoubledCoordinate` if necessary.
    pub const fn axial(&self) -> AxialCoordinate {
        match self {
            MapCoordinate::Axial(value) => *value,
            MapCoordinate::Doubled(value) => value.to_axial(),
        }
    }

    /// Returns a `DoubledCoordinate`, converting from `AxialCoordinate` if necessary.
    pub const fn doubled(&self) -> DoubledCoordinate {
        match self {
            MapCoordinate::Axial(value) => value.to_doubled(),
            MapCoordinate::Doubled(value) => *value,
        }
    }
}

impl From<AxialCoordinate> for MapCoordinate {
    fn from(value: AxialCoordinate) -> Self {
        MapCoordinate::Axial(value)
    }
}

impl From<DoubledCoordinate> for MapCoordinate {
    fn from(value: DoubledCoordinate) -> Self {
        MapCoordinate::Doubled(value)
    }
}

/// Axial coordinates for a hex grid.
/// Based on https://www.redblobgames.com/grids/hexagons/#conversions-axial
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxialCoordinate {
    pub q: i32,
    pub r: i32,
}

impl AxialCoordinate {
    pub const fn new(q: i32, r: i32) -> Self {
        AxialCoordinate { q, r }
    }

    pub const fn to_doubled(&self) -> DoubledCoordinate {
        let col = 2 * self.q + self.r;
        let row = self.r;
        return DoubledCoordinate::new(col, row);
    }

    pub fn neighbors(&self) -> [AxialCoordinate; 6] {
        let mut offsets = [
            AxialCoordinate::new(1, 0),
            AxialCoordinate::new(1, -1),
            AxialCoordinate::new(0, -1),
            AxialCoordinate::new(-1, 0),
            AxialCoordinate::new(-1, 1),
            AxialCoordinate::new(0, 1),
        ];

        for offset in &mut offsets {
            *offset += *self;
        }

        offsets
    }

    pub fn cartesian(&self) -> Vec2 {
        let sqrt3 = 3.0f32.sqrt();

        let x = 3.0 / 2.0 * self.q as f32;
        let y = sqrt3 / 2.0 * self.q as f32 + sqrt3 * self.r as f32;
        return Vec2::new(x, y);
    }
}

impl From<DoubledCoordinate> for AxialCoordinate {
    fn from(value: DoubledCoordinate) -> Self {
        value.to_axial()
    }
}

impl Add for AxialCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

impl AddAssign for AxialCoordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for AxialCoordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            q: self.q - rhs.q,
            r: self.r - rhs.r,
        }
    }
}

impl SubAssign for AxialCoordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

/// Double-width offset coordinates for a hex grid.
/// Based on https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct DoubledCoordinate { col: i32, row: i32 }

impl DoubledCoordinate {
    pub const fn new(col: i32, row: i32) -> Self {
        DoubledCoordinate { col, row }
    }
    
    pub const fn to_axial(&self) -> AxialCoordinate {
        let q = (self.col - self.row) / 2;
        let r = self.row;
        return AxialCoordinate::new(q, r);
    }

    pub fn neighbors(&self) -> [DoubledCoordinate; 6] {
        let mut offsets = [
            DoubledCoordinate::new(1, 0),
            DoubledCoordinate::new(1, -1),
            DoubledCoordinate::new(0, -1),
            DoubledCoordinate::new(-1, 0),
            DoubledCoordinate::new(-1, 1),
            DoubledCoordinate::new(0, 1),
        ];

        for offset in &mut offsets {
            *offset += *self;
        }

        offsets
    }
}

impl From<AxialCoordinate> for DoubledCoordinate {
    fn from(value: AxialCoordinate) -> Self {
        value.to_doubled()
    }
}

impl Add for DoubledCoordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col + rhs.col,
            row: self.row + rhs.row,
        }
    }
}

impl AddAssign for DoubledCoordinate {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Sub for DoubledCoordinate {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            col: self.col - rhs.col,
            row: self.row - rhs.row,
        }
    }
}

impl SubAssign for DoubledCoordinate {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}