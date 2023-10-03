use bevy::prelude::*;

const PIXEL_SIZE: f32 = 0.0;

/// Axial hex coordinate.
#[derive(Debug, Default, Hash, PartialEq, Eq, Reflect)]
pub struct HexCoordinate(pub UVec2);

impl HexCoordinate {
    pub fn to_pixel(&self) -> Vec2 {
        let x = PIXEL_SIZE * ( 3.0 / (2 * self.0.x) as f32);
        let y = PIXEL_SIZE * (3f32.sqrt() * (self.0.x as f32 + 3f32.sqrt()) * self.0.y as f32);
        Vec2 { x, y }
    }

    pub fn from_pixel(pixel: Vec2) -> Self {
        let q = ((3f32.sqrt() / 3.0) * pixel.x - 1./3. * pixel.y) / PIXEL_SIZE;
        let r = (2./3. * pixel.y) / PIXEL_SIZE;
        let r = 0.5 * (3f32.sqrt() * r - q);
        Self(UVec2 { x: q as u32, y: r as u32 })
    }
}