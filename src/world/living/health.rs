use bevy::prelude::*;
use crate::world::defs::species::{AssociatedSpecies, Species};
use super::{afflictions::{Afflicted, Affliction}, Living};

/// Component that caches the health value of a living creature, intended for fast reading.
/// This component is changed automatically when health factors change.
#[derive(Debug, Component, Clone)]
pub struct CachedHealth(f32);

impl CachedHealth {
    pub const fn new() -> Self {
        Self(f32::INFINITY)
    }

    #[allow(dead_code)]
    pub const fn read(&self) -> f32 {
        self.0
    }
}

/// Updates [CachedHealth] when its modifying values change.
pub(in super::super) fn health_caching_system(
    mut entities_query: Query<(&mut CachedHealth, Option<&AssociatedSpecies>, Option<&Afflicted>), Or<(Changed<AssociatedSpecies>, Changed<Afflicted>)>>,
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

/// Kills living things. What did you expect?
pub(in super::super) fn death_system(
    mut living: Query<&mut Living>,
    health: Query<(Entity, &CachedHealth), (With<Living>, Changed<CachedHealth>)>,
) {
    for (entity, health) in &health {
        let mut living = living.get_mut(entity).unwrap();
        if health.read() < 0.0 { *living = Living::Dead; }
    }
}