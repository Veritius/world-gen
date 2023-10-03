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

/// In-simulation point in time.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct SimulationInstant(pub u64);

impl SimulationInstant {
    pub fn since(&self, other: Self) -> Option<SimulationDuration> {
        match self.0.checked_sub(other.0) {
            Some(val) => Some(SimulationDuration(val)),
            None => None,
        }
    }

    pub fn since_saturating(&self, other: Self) -> SimulationDuration {
        SimulationDuration(self.0.saturating_sub(other.0))
    }
}

impl Numeric for SimulationInstant {
    const INTEGRAL: bool = true;

    const MIN: Self = Self(u64::MIN);

    const MAX: Self = Self(u64::MAX);

    fn to_f64(self) -> f64 {
        self.0 as f64
    }

    fn from_f64(num: f64) -> Self {
        Self(num as u64)
    }
}

impl Add<SimulationDuration> for SimulationInstant {
    type Output = Self;

    fn add(self, rhs: SimulationDuration) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub<SimulationDuration> for SimulationInstant {
    type Output = Self;

    fn sub(self, rhs: SimulationDuration) -> Self::Output {
        Self(self.0.saturating_sub(rhs.0))
    }
}

impl Display for SimulationInstant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let year = self.0 / 365;
        let day = self.0 % 365;
        f.write_str(&format!("{day}/{year}"))
    }
}

/// In-simulation span of time.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Reflect)]
pub struct SimulationDuration(pub u64);

impl SimulationDuration {
    pub fn days(days: u64) -> Self {
        Self(days)
    }

    pub fn years(years: u64) -> Self {
        Self(years * 365)
    }
}

impl Numeric for SimulationDuration {
    const INTEGRAL: bool = true;

    const MIN: Self = Self(u64::MIN);

    const MAX: Self = Self(u64::MAX);

    fn to_f64(self) -> f64 {
        self.0 as f64
    }

    fn from_f64(num: f64) -> Self {
        Self(num as u64)
    }
}

impl Add for SimulationDuration {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0)
    }
}

impl Sub for SimulationDuration {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0)
    }
}

impl Display for SimulationDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Get years and days
        let years = self.0 / 365;
        let days = self.0 % 365;

        // Calculate strings
        fn plurals(amt: u64, word: &'static str) -> Option<String> {
            match amt {
                0 => None,
                1 => Some(format!("{amt} {word}")),
                _ => Some(format!("{amt} {word}s")),
            }
        }

        // Create final string
        match (plurals(years, "year"), plurals(days, "day")) {
            (None, None) => f.write_str("0 days"),
            (None, Some(val)) => f.write_str(&val),
            (Some(val), None) => f.write_str(&val),
            (Some(a), Some(b)) => f.write_str(&format!("{a} and {b}")),
        }
    }
}