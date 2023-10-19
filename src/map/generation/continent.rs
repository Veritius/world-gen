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
        debug_assert_ne!(size.x, 0);
        debug_assert_ne!(size.y, 0);

        let mut commands = CommandQueue::default();
        let task_pool = TaskPool::new();

        // Divide task into fragments for concurrent processing
        fn fragment(x: u32) -> u32 {
            if x <= PROCESSING_FRAGMENT_SIZE { return 0 }
            if x % PROCESSING_FRAGMENT_SIZE == 0 {
                return (x / PROCESSING_FRAGMENT_SIZE) - 1
            }
            return x / PROCESSING_FRAGMENT_SIZE
        }

        let fx = fragment(size.x);
        let fy = fragment(size.y);

        debug_assert_eq!(fragment(64), 0);
        debug_assert_eq!(fragment(65), 1);
        debug_assert_eq!(fragment(128), 1);
        debug_assert_eq!(fragment(129), 2);

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
                s.spawn(async move {
                    // Function for determining how many cells should be generated in this task
                    fn defragment(i: u32, s: u32) -> u32 {
                        if s <= PROCESSING_FRAGMENT_SIZE { return s }
                        if fragment(s) > i { return PROCESSING_FRAGMENT_SIZE }
                        return s - (i * PROCESSING_FRAGMENT_SIZE)
                    }

                    // Determine how many cells in this fragment should be generated
                    let bx = defragment(x, size.x);
                    let by = defragment(y, size.y);
                });
            }
        });

        FinishedMapGenerationTask(commands)
    })
}