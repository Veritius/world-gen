//! A person in history.

use bevy::ecs::prelude::*;
use super::{common::{Name, Age}, living::Living};

#[derive(Bundle)]
pub struct PersonBundle {
    pub person: Person,
    pub personality: Personality,
    pub name: Name,
    pub age: Age,
    pub state: Living,
}

/// A marker component for a person in the world.
#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Personality {
    pub selflessness: f32,
    pub aggression: f32,
}

impl Personality {
    /// A tuple of mutable references to all the fields in this struct.
    pub fn split_borrow(&mut self) -> (&mut f32, &mut f32) {
        (
            &mut self.selflessness,
            &mut self.aggression
        )
    }
}

impl Default for Personality {
    fn default() -> Self {
        const MIDPOINT: f32 = 0.5;

        Self {
            selflessness: MIDPOINT,
            aggression: MIDPOINT,
        }
    }
}