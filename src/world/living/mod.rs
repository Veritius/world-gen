//! Components for living creatures.

pub mod afflictions;

use bevy::prelude::*;
use self::afflictions::Affliction;

/// Anything with this component will be considered 'living' and its behavior will change.
/// This includes age not incrementing when dead.
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub enum Living {
    Alive,
    Dead,
}

#[derive(Debug, Component)]
pub struct Afflicted(pub Vec<Affliction>);