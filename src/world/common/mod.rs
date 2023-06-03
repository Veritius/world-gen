//! Qualities common to a lot of entities, like their name or age.

use bevy::ecs::prelude::*;
use super::{living::Living, time::Age, defs::{SimulationConfig, Timespan}};

/// Any entities with this component will have more in-depth information generated.
#[derive(Component, Clone)]
pub struct Important;

#[derive(Component, Clone)]
pub struct Name(pub String);

impl<T: Into<String>> From<T> for Name {
    fn from(value: T) -> Self {
        Name(value.into())
    }
}

/// Increments the age value each tick.
pub(super) fn age_incrementor_system(
    config: Res<SimulationConfig>,
    mut query: Query<(&mut Age, Option<&Living>)>,
) {
    for (mut age, status) in query.iter_mut() {
        // Don't age dead things.
        if status.is_some() && *status.unwrap() == Living::Dead { continue; }

        match config.timespan {
            Timespan::Months => age.add_months(1),
            Timespan::Days => age.add_days(1),
        }
    }
}