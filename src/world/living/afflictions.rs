//! Values for calculating health.

use std::{fmt::Debug, collections::{BTreeMap, btree_map::Iter}};
use bevy::prelude::{Component, Entity, Bundle, Query, Res};
use crate::world::{common::Name, defs::{SimulationConfig, Timespan}};

/// A value for an affliction that changes depending on severity.
#[derive(Debug)]
pub enum SeverityVariableValue {
    /// Equal to `Static(0.0)` as a flat rate and `Static(1.0)` as a coefficient.
    NoAdjustment,
    /// Multiplied by severity to get amount.
    Scaling(f32),
    /// Always remains this value and ignores severity.
    Static(f32),
    /// A Rust function that accepts severity and outputs the new value.
    Custom(Box<dyn SeverityVariableFn>),
}

impl SeverityVariableValue {
    /// Takes a severity value and whether or not it is a coefficient, and returns a value that should be added to health to apply it.
    pub fn effect(&self, coefficient: bool, severity: f32) -> f32 {
        match self {
            SeverityVariableValue::NoAdjustment => { if coefficient == true { return 1.0 } else { return 0.0 } },
            SeverityVariableValue::Scaling(value) => { return value * severity },
            SeverityVariableValue::Static(value) => { return *value },
            SeverityVariableValue::Custom(value) => { return value(severity) },
        }
    }
}

/// Auto-trait for [SeverityVariableValue]
pub trait SeverityVariableFn: Debug + Send + Sync + Fn(f32) -> f32 {}
impl<T: Debug + Send + Sync + Fn(f32) -> f32> SeverityVariableFn for T {}

/// Defines an 'affliction' a living creature can obtain.
/// Use the [Afflicted] component and insert the entity ID to add an affliction to something, don't put this component on them.
// (unless you want them to become the physical embodiment of a disease)
///
/// Afflictions are applied in a specific but nondeterministic order.
/// First, all the flat rate changes are applied.
/// Second, all the coefficients are applied.
/// The cached `Health` value is based on the associated species of the creature.
#[derive(Debug, Component)]
pub struct Affliction {
    /// Added to a value that is later added to the health.
    pub flat: SeverityVariableValue,
    /// Applied to the flat rate modifier after they're summed up.
    pub coefficient: SeverityVariableValue,
    /// Defines the speed of progression for this disease.
    /// This is applied every tick to values in [Afflicted] based on days. If the sim timespan is months, it's multiplied by 30.
    pub progression_speed: SeverityVariableValue,
}

impl Default for Affliction {
    fn default() -> Self {
        Self {
            flat: SeverityVariableValue::NoAdjustment,
            coefficient: SeverityVariableValue::NoAdjustment,
            progression_speed: SeverityVariableValue::NoAdjustment,
        }
    }
}

#[derive(Bundle)]
pub struct AfflictionBundle {
    pub name: Name,
    pub affliction: Affliction,
}

#[derive(Debug, Component)]
pub struct Afflicted(BTreeMap<Entity, f32>);

impl Afflicted {
    pub fn iter(&self) -> Iter<Entity, f32> {
        self.0.iter()
    }
}

pub(in super::super) fn affliction_progress_system(
    config: Res<SimulationConfig>,
    afflictions: Query<&Affliction>,
    mut afflicted: Query<&mut Afflicted>,
) {
    for mut afflicted in afflicted.iter_mut() {
        for (k, v) in afflicted.0.iter_mut() {
            if let Ok(affliction) = afflictions.get(*k) {
                // Calculate affliction change
                let mut adjust = affliction.progression_speed.effect(false, *v);
                if adjust == 0.0 { continue; }
                if config.timespan == Timespan::Months { adjust *= 30.0; }

                // Apply change
                *v += adjust;
            }
        }
    }
}