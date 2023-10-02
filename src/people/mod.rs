use bevy::prelude::*;
use crate::common::DisplayName;
use self::personality::Personality;

pub mod personality;

#[derive(Debug, Bundle)]
pub struct PersonBundle {
    pub name: DisplayName,
    pub personality: Personality,
}