//! Measurements of time within the simulation.

use std::{ops::{Add, Sub}, fmt::Display};
use bevy::prelude::*;
use bevy_egui::egui::emath::Numeric;

/// Information about the simulation's current date.
#[derive(Debug, Default, Clone, Copy, Resource, Reflect)]
pub struct SimulationTime {
    pub start_day: SimulationInstant,
    pub current_day: SimulationInstant,
}

/// Stores the day the object came into existence. Use with [SimulationTime] to get the age of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct CreationDate(pub SimulationInstant);

/// In-simulation point in time. Can be negative.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct SimulationInstant(i64);

impl SimulationInstant {
    pub fn since(&self, other: Self) -> Option<SimulationDuration> {
        match self.0.checked_sub(other.0) {
            Some(val) => Some(SimulationDuration(val as u64)),
            None => None,
        }
    }

    pub fn since_saturating(&self, other: Self) -> SimulationDuration {
        SimulationDuration(self.0.saturating_sub(other.0) as u64)
    }
}

impl Numeric for SimulationInstant {
    const INTEGRAL: bool = true;

    const MIN: Self = Self(i64::MIN);

    const MAX: Self = Self(i64::MAX);

    fn to_f64(self) -> f64 {
        self.0 as f64
    }

    fn from_f64(num: f64) -> Self {
        Self(num as i64)
    }
}

impl Add<SimulationDuration> for SimulationInstant {
    type Output = Self;

    fn add(self, rhs: SimulationDuration) -> Self::Output {
        Self(self.0 + rhs.0 as i64)
    }
}

impl Sub<SimulationDuration> for SimulationInstant {
    type Output = Self;

    fn sub(self, rhs: SimulationDuration) -> Self::Output {
        Self(self.0 - rhs.0 as i64)
    }
}

/// In-simulation span of time. Cannot be negative.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct SimulationDuration(u64);

impl SimulationDuration {
    pub fn days(days: u64) -> Self {
        Self(days)
    }

    pub fn years(years: u64) -> Self {
        Self(years * 365)
    }
}

impl Display for SimulationDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let years = self.0 / 365;
        let days = self.0 % 365;
        f.write_str(&format!("{years} years and {days} days"))
    }
}