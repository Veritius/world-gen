//! Values for calculating health.

use std::{fmt::Debug, collections::BTreeMap};
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

/// Auto-trait for [HealthAdjustmentFunction]
pub trait HealthAdjustFn: Debug + Send + Sync + Fn(f32) -> f32 {}
impl<T: Debug + Send + Sync + Fn(f32) -> f32> HealthAdjustFn for T {}

/// Defines an 'affliction' a living creature can obtain.
/// Use the [Afflicted] component and insert the entity ID to add an affliction to something, don't put this component on them.
// (unless you want them to become the physical embodiment of a disease)
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