use std::ops::RangeInclusive;
use bevy::prelude::*;

pub const PERSONALITY_VALUE_RANGE: RangeInclusive<f32> = -10.0..=10.0;

/// Personality information for a living creature.
#[derive(Debug, Default, Clone, Component, Reflect)]
pub struct Personality {
    pub selflessness: f32,
    pub aggression: f32,
}