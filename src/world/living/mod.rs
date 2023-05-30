//! Components for living creatures.

use bevy::prelude::*;

/// Anything with this component will be considered 'living' and its behavior will change.
/// This includes age not incrementing when dead.
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub enum Living {
    Alive,
    Dead,
}