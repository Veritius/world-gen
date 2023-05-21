//! Places in history.

use bevy_ecs::prelude::*;
use bevy_hierarchy::Parent;
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

/// A bundle for creating settlements/towns/cities. If the settlement is part of a region, use `RegionalSettlementBundle`.
#[derive(Bundle)]
pub struct SettlementBundle {
    pub name: Name,
    pub settlement: Settlement,
}

/// A bundle for creating settlements/towns/cities. This one has a `Parent` component for making it part of a `Region`.
#[derive(Bundle)]
pub struct RegionalSettlementBundle {
    pub name: Name,
    pub settlement: Settlement,
    pub region: Parent,
}

/// A discrete settlement, town, or city.
/// Put this on an entity that is a child of an entity with a `Region` component to start defining nations.
#[derive(Component)]
pub struct Settlement {
    pub population: u32,
}