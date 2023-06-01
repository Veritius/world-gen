//! Values for calculating health.

use std::{fmt::Debug, collections::{BTreeMap, btree_map::Iter}};
use bevy::prelude::{Component, Entity, Bundle};
use crate::world::common::Name;

#[derive(Debug)]
pub enum HealthAdjustmentFunction {
    /// Equal to `Static(0.0)` as a flat rate and `Static(1.0)` as a coefficient.
    NoAdjustment,
    /// Multiplied by severity to get amount.
    Scaling(f32),
    /// Always remains this value and ignores severity.
    Static(f32),
    /// A Rust function that accepts severity and outputs the new value.
    Custom(Box<dyn HealthAdjustFn>),
}

impl HealthAdjustmentFunction {
    /// Takes a severity value and whether or not it is a coefficient, and returns a value that should be added to health to apply it.
    pub fn effect(&self, coefficient: bool, severity: f32) -> f32 {
        match self {
            HealthAdjustmentFunction::NoAdjustment => { if coefficient == true { return 1.0 } else { return 0.0 } },
            HealthAdjustmentFunction::Scaling(value) => { return value * severity },
            HealthAdjustmentFunction::Static(value) => { return *value },
            HealthAdjustmentFunction::Custom(value) => { return value(severity) },
        }
    }
}

/// Auto-trait for [HealthAdjustmentFunction]
pub trait HealthAdjustFn: Debug + Send + Sync + Fn(f32) -> f32 {}
impl<T: Debug + Send + Sync + Fn(f32) -> f32> HealthAdjustFn for T {}

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
    pub flat: HealthAdjustmentFunction,
    pub coefficient: HealthAdjustmentFunction,
}

impl Default for Affliction {
    fn default() -> Self {
        Self {
            flat: HealthAdjustmentFunction::NoAdjustment,
            coefficient: HealthAdjustmentFunction::NoAdjustment,
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