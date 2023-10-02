use bevy::prelude::*;
use crate::common::{DisplayName, Age};

#[derive(Debug, Component)]
pub struct Faction;

#[derive(Debug, Bundle)]
pub struct FactionBundle {
    pub marker: Faction,
    pub name: DisplayName,
    pub age: Age,
}