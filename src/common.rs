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

#[derive(Debug, Default, Clone, Copy, Resource, Reflect)]
pub struct SimulationTime {
    pub passed_days: u64,
}

impl SimulationTime {
    pub fn current_day(&self) -> u64 {
        self.passed_days
    }
    
    pub fn get_age(&self, birthday: Birthday) -> Option<u64> {
        self.passed_days.checked_sub(birthday.0)
    }

    pub fn get_age_str(&self, birthday: Birthday) -> String {
        let age = self.get_age(birthday);
        if age.is_none() { return "Older than time".to_string(); }
        let age = age.unwrap();
        let years = age / 365;
        let days = age % 365;
        match (years, days) {
            (0, 1) => format!("{} day", days),
            (0, _) => format!("{} days", days),
            (1, 1) => format!("{} year and {} day", years, days),
            (1, _) => format!("{} year and {} days", years, days),
            (_, _) => format!("{} years and {} days", years, days),
        }
    }
}

/// Stores the day the object came into existence. Use with [SimulationTime] to get the age of an entity.
#[derive(Debug, Clone, Copy, PartialEq, Component, Reflect)]
pub struct Birthday(pub u64);

impl Birthday {
    pub fn from_days(days: u64) -> Self {
        Self(days as u64)
    }

    pub fn from_years(years: u32) -> Self {
        Self(years as u64 * 365)
    }
}