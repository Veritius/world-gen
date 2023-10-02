use std::ops::RangeInclusive;
use bevy::prelude::*;
use crate::{common::DisplayName, time::CreationDate};

pub const FACTION_INTEREST_RANGE: RangeInclusive<f32> = -10.0..=10.0;


#[derive(Debug, Default, Component)]
pub struct Faction {
    pub profit_interest: f32,
    pub expansion_interest: f32,
    pub humanitarian_interest: f32,
}

#[derive(Debug, Bundle)]
pub struct FactionBundle {
    pub marker: Faction,
    pub name: DisplayName,
    pub age: CreationDate,
}