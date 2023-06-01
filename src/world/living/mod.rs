//! Components for living creatures.

pub mod afflictions;

use bevy::prelude::*;
use self::afflictions::{Afflicted, Affliction};
use super::defs::species::{AssociatedSpecies, Species};

/// Anything with this component will be considered 'living' and its behavior will change.
/// This includes age not incrementing when dead.
#[derive(Debug, Component, Clone, PartialEq, Eq)]
pub enum Living {
    Alive,
    Dead,
}

/// Component that caches the health value of a living creature, intended for fast reading.
/// This component is changed automatically when health factors change.
#[derive(Debug, Component, Clone)]
pub struct Health(f32);

impl Health {
    pub const fn new() -> Self {
        Self(f32::INFINITY)
    }

    #[allow(dead_code)]
    pub const fn read(&self) -> f32 {
        self.0
    }
}

pub(super) fn health_caching_system(
    mut entities_query: Query<(&mut Health, Option<&AssociatedSpecies>, Option<&Afflicted>), Or<(Changed<AssociatedSpecies>, Changed<Afflicted>)>>,
    // TODO: Re-cache if affliction or species definitions change.
    afflictions: Query<&Affliction>,
    species_query: Query<&Species>,
) {
    for (mut health, species, afflicted) in entities_query.iter_mut() {
        let mut adjust: f32 = 0.0;

        let mut coefficient: f32 = 1.0;

        if let Some(afflicted) = afflicted {
            for (id, severity) in afflicted.iter() {
                let q = afflictions.get(*id);
                if q.is_err() { continue; }
                let affliction = q.unwrap();

                // Apply flat effects first
                adjust += affliction.flat.effect(false, *severity);
                coefficient *= affliction.coefficient.effect(true, *severity);
            }
        }

        // Apply coefficients
        adjust *= coefficient;

        if let Some(species) = species {
            let q = species_query.get(species.0);
            if q.is_ok() {
                health.0 = q.unwrap().resilience * adjust;
            } else {
                health.0 = 100.0 * adjust;
            }
        }
    }
}