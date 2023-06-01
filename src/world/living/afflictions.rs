//! Values for calculating health.

use std::fmt::Debug;

#[derive(Debug)]
pub enum HealthAdjustmentFunction {
    Static(f32),
    Custom(Box<dyn HealthAdjustFn>),
}

/// Auto-trait for [HealthAdjustmentFunction]
pub trait HealthAdjustFn: Debug + Send + Sync + Fn(f32) -> f32 {}
impl<T: Debug + Send + Sync + Fn(f32) -> f32> HealthAdjustFn for T {}

#[derive(Debug)]
pub struct Affliction {
    pub name: String,
    pub severity: f32,
    pub flat: HealthAdjustmentFunction,
    pub coefficient: HealthAdjustmentFunction,
}