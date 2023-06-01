//! In-simulation time.

use std::{fmt::Display, str::FromStr};
use bevy::prelude::Component;
use eframe::emath::Numeric;

/// Tracks time in days.
#[derive(Component, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Age(u32);

impl Age {
    pub const ZERO: Age = Age(0);

    /// Returns a tuple of (days, months, years)
    pub fn am_tuple(&self) -> (u32, u32, u32) {
        let val = self.days_passed();
        let years = val / 360;
        let months = (val % 360) / 30;
        let days = val % 360 % 30;

        (days, months, years)
    }

    /// Returns how many years have passed.
    pub fn years_passed(&self) -> u32 {
        self.0 / 360
    }

    pub const fn from_years(years: u32) -> Self {
        Age(years * 360)
    }

    /// Returns how many months have passed.
    pub fn months_passed(&self) -> u32 {
        self.0 / 30
    }

    pub const fn from_months(months: u32) -> Self {
        Age(months * 30)
    }

    /// Returns how many days have passed.
    pub fn days_passed(&self) -> u32 {
        self.0
    }

    pub const fn from_days(days: u32) -> Self {
        Age(days)
    }

    pub fn add_years(&mut self, amount: u32) {
        self.0 += amount * 360;
    }

    pub fn add_months(&mut self, amount: u32) {
        self.0 += amount * 30;
    }

    pub fn add_days(&mut self, amount: u32) {
        self.0 += amount;
    }
}

impl Numeric for Age {
    const INTEGRAL: bool = true;

    const MIN: Self = Age::ZERO;

    const MAX: Self = Age(u32::MAX);

    fn to_f64(self) -> f64 {
        self.0.to_f64()
    }

    fn from_f64(num: f64) -> Self {
        Self::from_days(num as u32)
    }
}

impl Display for Age {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (days, months, years) = self.am_tuple();

        if years != 0 { f.write_str(&format!("{years} years ")).unwrap(); }
        if months != 0 { f.write_str(&format!("{months} months ")).unwrap(); }
        f.write_str(&format!("{days} days")).unwrap();
        
        Ok(())
    }
}

impl FromStr for Age {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut time = Age::ZERO;

        let split: Vec<&str> = s.split(' ').collect();
        for chunk in split.chunks(2) {
            if chunk.len() != 2 { return Err(()); }

            let num = u32::from_str(chunk[0]);
            if num.is_err() { return Err(()); }
            let num = num.unwrap();

            let val = chunk[1];

            match val {
                "years" | "year" | "y" => { time.add_years(num); },
                "months" | "month" | "m" => { time.add_months(num); },
                "days" | "day" | "d" => { time.add_days(num); }
                _ => { return Err(()); }
            }
        }

        Ok(time)
    }
}