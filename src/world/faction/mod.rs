//! "Factions" aka groups that people can be aligned with.

use std::collections::BTreeMap;
use bevy::prelude::*;
use super::person::Personality;

#[allow(dead_code)]
#[derive(Component)]
pub struct Faction {
    /// An offset to the individual personalities of its members.
    personality_offset: Personality,

    /// What this faction thinks of other factions.
    relations: BTreeMap<Entity, f32>,
}

/// This entity is a member of a faction or factions.
#[derive(Component)]
pub struct FactionMember(Vec<Faction>);