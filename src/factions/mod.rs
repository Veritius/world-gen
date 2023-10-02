use bevy::prelude::*;

#[derive(Debug, Component)]
pub struct Faction;

#[derive(Debug, Bundle)]
pub struct FactionBundle {
    pub marker: Faction,
}