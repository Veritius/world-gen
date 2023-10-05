use bevy::{prelude::*, tasks::{Task, TaskPool, AsyncComputeTaskPool}, ecs::system::CommandQueue};
use noise::{Fbm, Perlin, NoiseFn};
use super::FinishedMapGenerationTask;

const PROCESSING_FRAGMENT_SIZE: u32 = 64;

const SEA_LEVEL: f64 = 0.0;

pub(super) fn single_continent(
    tasks: &AsyncComputeTaskPool,
    seed: u32,
    size: UVec2,
) -> Task<FinishedMapGenerationTask> {
    tasks.spawn(async move {
        let mut commands = CommandQueue::default();
        let task_pool = TaskPool::new();

        // Divide task into fragments for concurrent processing
        fn fragment(x: u32) -> u32 { (x / PROCESSING_FRAGMENT_SIZE) + if (x % PROCESSING_FRAGMENT_SIZE) > 0 { 1 } else { 0 } }
        let fx = fragment(size.x);
        let fy = fragment(size.y);

        // Iterator that returns all fragments
        let iter = (0..=fx)
            .map(move |w| (0..=fy)
            .map(move |h| (w, h)))
            .flatten();

        // Noise layers for fragments to use
        let height_layer = Fbm::<Perlin>::new(seed);

        // Tasks for all fragments
        task_pool.scope(|s| {
            for (x, y) in iter {
                s.spawn(async {
                    todo!()
                });
            }
        });

        FinishedMapGenerationTask(commands)
    })
}