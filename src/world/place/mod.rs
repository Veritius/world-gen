//! Places in history.

use bevy::ecs::prelude::*;
use super::thing::Name;

/// A bundle for creating regions.
/// Parent regions must be added manually.
#[derive(Bundle)]
pub struct RegionBundle {
    pub name: Name,
    pub area: Region,
}

/// A general location, like a territory, country, or continent.
/// This is combined with Bevy parenting to define hierarchies.
#[derive(Component)]
pub struct Region;

/// A bundle for creating settlements/towns/cities.
#[derive(Bundle)]
pub struct SettlementBundle {
    pub name: Name,
    pub settlement: Settlement,
}

/// A discrete settlement, town, or city.
/// Put this on an entity that is a child of an entity with a `Region` component to start defining nations.
#[derive(Component)]
pub struct Settlement {
    pub population: u32,
}

impl Default for Settlement {
    fn default() -> Self {
        Self {
            population: 0
        }
    }
}