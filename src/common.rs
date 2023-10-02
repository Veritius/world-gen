use std::fmt::Display;
use bevy::prelude::*;

/// Entities with this component will (probably) not be processed as part of the simulation.
#[derive(Debug, Clone, Component)]
pub struct Suspended;

/// A name for an in-simulation entity.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
pub struct DisplayName(pub String);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

/// An object's age in days.
#[derive(Debug, Clone, PartialEq, Component, Reflect)]
pub struct Age(pub u64);

impl Age {
    pub fn from_days(days: u64) -> Self {
        Self(days as u64)
    }

    pub fn from_years(years: u32) -> Self {
        Self(years as u64 * 365)
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let years = self.0 / 365;
        let days = self.0 % 365;
        match (years, days) {
            (0, 1) => f.write_str(&format!("{} day", days)),
            (0, _) => f.write_str(&format!("{} days", days)),
            (1, 1) => f.write_str(&format!("{} year and {} day", years, days)),
            (1, _) => f.write_str(&format!("{} year and {} days", years, days)),
            (_, _) => f.write_str(&format!("{} years and {} days", years, days)),
        }
    }
}