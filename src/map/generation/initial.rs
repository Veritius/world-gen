use std::sync::{Arc, Mutex};
use bevy::{prelude::*, tasks::AsyncComputeTaskPool};
use crate::map::{SimulationMap, cells::{MapLayer, MapCell}};
use super::{RegenerateMapEvent, WorldGenerationMethod, terrain::single_continent, RunningMapGenerationTask, MapGenerationTaskStatus};

pub fn generation_dispatch_system(
    mut commands: Commands,
    mut regenerate_events: EventReader<RegenerateMapEvent>,
    gen_config: Res<SimulationMap>,
    map_entities: Query<Entity, Or<(With<MapLayer>, With<MapCell>)>>,
) {
    // Despawn all map entities
    if !regenerate_events.is_empty() {
        regenerate_events.clear();
        for entity in map_entities.iter() {
            commands.entity(entity).despawn();
        }
    } else {
        return;
    }

    // Create task
    let tasks = AsyncComputeTaskPool::get();
    let completion = Arc::new(Mutex::new((0.0, "Starting".to_string())));
    let task = match gen_config.gen_method {
        WorldGenerationMethod::SingleContinent => single_continent(tasks, gen_config.random_seed, gen_config.map_size),
    };

    // Add resource
    commands.insert_resource(RunningMapGenerationTask {
        completion,
        task: MapGenerationTaskStatus::Terrain(task)
    });
}