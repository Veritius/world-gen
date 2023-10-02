use bevy::prelude::*;
use crate::common::{DisplayName, Birthday};
use self::personality::Personality;

pub mod personality;

/// Marker component for a person.
#[derive(Debug, Component)]
pub struct Person;

#[derive(Debug, Bundle)]
pub struct PersonBundle {
    pub marker: Person,
    pub name: DisplayName,
    pub age: Birthday,
    pub personality: Personality,
}