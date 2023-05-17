//! A person in history.

use bevy_ecs::prelude::*;
use super::thing::{Name, Age};

#[derive(Bundle)]
pub struct PersonBundle {
    pub person: Person,
    pub name: Name,
    pub age: Age,
}


/// A marker component for a person in the world.
#[derive(Component)]
pub struct Person;