use bevy::prelude::*;
use crate::map::{SimulationMap, cells::MapLayer};
use super::RegenerateMapEvent;

pub fn generation_dispatch_system(
    mut commands: Commands,
    mut regenerate_events: EventReader<RegenerateMapEvent>,
    gen_config: Res<SimulationMap>,
    layers: Query<Entity, With<MapLayer>>,
) {

}