//! A person in history.

use bevy_ecs::prelude::*;


/// A marker component for a person in the world.
#[derive(Component)]
pub struct Person;

#[derive(Component)]
pub struct Name(pub Vec<String>);