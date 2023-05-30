//! Qualities common to a lot of entities, like their name or age.

use bevy::ecs::prelude::*;
use super::{living::Living, time::TimeLength, defs::{SimulationConfig, Timespan}};

/// Any entities with this component will have more in-depth information generated.
#[derive(Component, Clone)]
pub struct Important;

#[derive(Component, Clone)]
pub struct Name(pub String);

/// The 'age' of something. If the entity this is attached to has a [Living] component that is `Dead`, the age will not increment.
#[derive(Component, Clone)]
pub struct Age(pub TimeLength);

/// Increments the age value each tick.
pub(super) fn age_incrementor_system(
    config: Res<SimulationConfig>,
    mut query: Query<(&mut Age, Option<&Living>)>,
) {
    for (mut age, status) in query.iter_mut() {
        // Don't age dead things.
        if status.is_some() && *status.unwrap() == Living::Dead { continue; }

        match config.timespan {
            Timespan::Months => age.0.add_months(1),
            Timespan::Days => age.0.add_days(1),
        }
    }
}