use bevy::prelude::*;
use crate::{time::SimulationDuration, common::DisplayName};

#[derive(Debug, Component)]
pub struct Species {
    pub is_humanoid: bool,
    pub age_of_maturity: SimulationDuration,
}

impl Species {
    pub fn humanoid_default() -> Self {
        Self {
            is_humanoid: true,
            age_of_maturity: SimulationDuration::years(18),
        }
    }

    pub fn animal_default() -> Self {
        Self {
            is_humanoid: false,
            age_of_maturity: SimulationDuration::years(1),
        }
    }
}

#[derive(Debug, Bundle)]
pub struct SpeciesBundle {
    pub species: Species,
    pub name: DisplayName,
}