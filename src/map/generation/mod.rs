pub(in super::super) mod initial;

use bevy::prelude::*;

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorldGenerationMethod {
    #[default]
    SingleContinent,
}

/// Raise to clear all map cells and regenerate the world.
#[derive(Debug, Event)]
pub struct RegenerateMapEvent;