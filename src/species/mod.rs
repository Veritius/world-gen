use bevy::prelude::*;
use crate::{time::SimulationDuration, common::DisplayName};

#[derive(Debug, Default, Component)]
pub struct Species {
    pub is_humanoid: bool,
    pub age_of_maturity: SimulationDuration,
}

#[derive(Debug, Bundle)]
pub struct SpeciesBundle {
    pub species: Species,
    pub name: DisplayName,
}