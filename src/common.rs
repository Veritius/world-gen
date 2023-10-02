use bevy::prelude::*;

/// A name for an in-simulation entity.
#[derive(Debug, Clone, PartialEq, Eq, Component, Reflect)]
pub struct DisplayName(pub String);

impl DisplayName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}