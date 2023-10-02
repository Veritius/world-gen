use bevy::prelude::*;
use crate::{common::DisplayName, time::CreationDate};
use self::personality::Personality;

pub mod personality;

/// Marker component for a person.
#[derive(Debug, Component)]
pub struct Person;

#[derive(Debug, Bundle)]
pub struct PersonBundle {
    pub marker: Person,
    pub name: DisplayName,
    pub age: CreationDate,
    pub personality: Personality,
}