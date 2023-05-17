use bevy_ecs::prelude::{Component, Bundle};
use super::thing::Name;

#[derive(Bundle)]
pub struct SpeciesBundle {
    pub name: Name,
    pub species: Species,
}

#[derive(Component)]
pub struct Species {
    /// The age at which this creature is considered fully formed or matured.
    pub maturity_age: u32,
    /// The maximum age at which this creature will die. Approaching this age will increase harm to the creature.
    pub max_age: u32,
}