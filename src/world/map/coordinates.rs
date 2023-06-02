use bevy::prelude::*;

/// A hex map coordinate that can be in cube or offset form.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum MapCoordinate {
    /// Cube hex coordinates.
    Cube(CubeCoordinate),
    /// Offset hex coordinates (doubled!)
    Offset(OffsetCoordinate),
}

impl MapCoordinate {
    /// Returns a `CubeCoordinate`, converting from `OffsetCoordinate` if necessary.
    pub fn cube(&self) -> CubeCoordinate {
        match self {
            MapCoordinate::Cube(value) => *value,
            MapCoordinate::Offset(value) => value.to_cube(),
        }
    }

    /// Returns an `OffsetCoordinate`, converting from `CubeCoordinate` if necessary.
    pub fn offset(&self) -> OffsetCoordinate {
        match self {
            MapCoordinate::Cube(value) => value.to_offset(),
            MapCoordinate::Offset(value) => *value,
        }
    }
}

impl From<CubeCoordinate> for MapCoordinate {
    fn from(value: CubeCoordinate) -> Self {
        MapCoordinate::Cube(value)
    }
}

impl From<OffsetCoordinate> for MapCoordinate {
    fn from(value: OffsetCoordinate) -> Self {
        MapCoordinate::Offset(value)
    }
}

/// "Cube" coordinates for a hex grid.
/// Based on https://www.redblobgames.com/grids/hexagons/#coordinates-cube
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct CubeCoordinate(IVec3);

impl CubeCoordinate {
    pub fn to_offset(&self) -> OffsetCoordinate {
        let col = self.0.x;
        let row = 2 * self.0.y + self.0.x;
        return OffsetCoordinate(IVec2::new(col, row));
    }

    pub fn neighbors(&self) -> [CubeCoordinate; 6] {
        let mut precomp = [
            CubeCoordinate(IVec3::new(1, 0, -1)),
            CubeCoordinate(IVec3::new(1, -1, 0)),
            CubeCoordinate(IVec3::new(0, -0, 1)),
            CubeCoordinate(IVec3::new(-1, 0, 1)),
            CubeCoordinate(IVec3::new(-1, 1, 0)),
            CubeCoordinate(IVec3::new(0, 1, -1)),
        ];

        for x in &mut precomp {
            x.0 += self.0;
        }

        precomp
    }
}

/// Double-width offset coordinates for a hex grid.
/// Based on https://www.redblobgames.com/grids/hexagons/#coordinates-doubled
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct OffsetCoordinate(IVec2);

impl OffsetCoordinate {
    pub fn to_cube(&self) -> CubeCoordinate {
        let q = (self.0.x - self.0.y) / 2;
        let r = self.0.y;
        return CubeCoordinate(IVec3::new(q, r, -q-r));
    }

    pub fn neighbors(&self) -> [OffsetCoordinate; 6] {
        let mut precomp = [
            OffsetCoordinate(IVec2::new(2, 0)),
            OffsetCoordinate(IVec2::new(1, -1)),
            OffsetCoordinate(IVec2::new(-1, -1)),
            OffsetCoordinate(IVec2::new(-2, 0)),
            OffsetCoordinate(IVec2::new(-1, 1)),
            OffsetCoordinate(IVec2::new(1, 1)),
        ];

        for x in &mut precomp {
            x.0 += self.0;
        }

        precomp
    }
}