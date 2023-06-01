use bevy::ecs::prelude::*;
use crate::world::{common::Name, time::Age};

#[derive(Bundle)]
pub struct SpeciesBundle {
    pub name: Name,
    pub species: Species,
}

#[derive(Component, Clone)]
pub struct Species {
    /// Is this species humanoid?
    pub humanoid: bool,
    /// The age at which this creature is considered fully formed or matured.
    pub maturity_age: Age,
    /// The maximum age at which this creature will die. Approaching this age will increase harm to the creature.
    pub max_age: Age,
    /// The maximum health an entity of this species has.
    pub resilience: f32,
}

#[derive(Component, Clone, PartialEq, Eq)]
pub struct AssociatedSpecies(pub Entity);