//! A person in history.

use bevy::ecs::prelude::*;
use super::{common::Name, living::{Living, health::CachedHealth}, time::Age};

#[derive(Bundle)]
pub struct PersonBundle {
    pub person: Person,
    pub personality: Personality,
    pub name: Name,
    pub age: Age,
    pub state: Living,
    pub health: CachedHealth,
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
    /// Returns a struct that allows mutable access to multiple personality values at once.
    pub fn multiple_borrow(&mut self) -> PersonalityMutRef {
        PersonalityMutRef {
            selflessness: &mut self.selflessness,
            aggression: &mut self.aggression,
        }
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

/// Mutable references to all personality values.
pub struct PersonalityMutRef<'a> {
    pub selflessness: &'a mut f32,
    pub aggression: &'a mut f32,
}