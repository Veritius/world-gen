use bevy::prelude::*;
use crate::common::{DisplayName, Age};
use self::personality::Personality;

pub mod personality;

#[derive(Debug, Bundle)]
pub struct PersonBundle {
    pub name: DisplayName,
    pub age: Age,
    pub personality: Personality,
}