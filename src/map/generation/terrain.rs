use bevy::{prelude::*, tasks::{Task, TaskPool, AsyncComputeTaskPool}, ecs::system::CommandQueue};
use fastrand::Rng;
use super::FinishedMapGenerationTask;

const PROCESSING_FRAGMENT_SIZE: u32 = 64;
const SEA_LEVEL: f64 = 0.0;

// Divide task into fragments for concurrent processing
fn fragment(x: u32) -> u32 {
    if x <= PROCESSING_FRAGMENT_SIZE { return 0 }
    if x % PROCESSING_FRAGMENT_SIZE == 0 {
        return (x / PROCESSING_FRAGMENT_SIZE) - 1
    }
    return x / PROCESSING_FRAGMENT_SIZE
}

// Determine how many cells to generate per fragment, and the coordinate offset
fn defragment(i: u32, s: u32) -> (u32, u32) {
    if s <= PROCESSING_FRAGMENT_SIZE { return (s, 0) }
    if fragment(s) > i { return (PROCESSING_FRAGMENT_SIZE, PROCESSING_FRAGMENT_SIZE * i) }
    return (
        s - (i * PROCESSING_FRAGMENT_SIZE),
        (i * PROCESSING_FRAGMENT_SIZE),
    )
}

#[test]
fn check_fragmentation_math() {
    assert_eq!(fragment(PROCESSING_FRAGMENT_SIZE - 1), 0);
    assert_eq!(fragment(PROCESSING_FRAGMENT_SIZE), 0);
    assert_eq!(fragment(PROCESSING_FRAGMENT_SIZE + 1), 1);
    assert_eq!(fragment(PROCESSING_FRAGMENT_SIZE * 2), 1);
    assert_eq!(fragment((PROCESSING_FRAGMENT_SIZE * 2) + 1), 2);

    assert_eq!(defragment(0, PROCESSING_FRAGMENT_SIZE - 1), (PROCESSING_FRAGMENT_SIZE - 1, 0));
    assert_eq!(defragment(0, PROCESSING_FRAGMENT_SIZE), (PROCESSING_FRAGMENT_SIZE, 0));
    assert_eq!(defragment(1, PROCESSING_FRAGMENT_SIZE + 1), (1, PROCESSING_FRAGMENT_SIZE));
}

fn generic_continent_generation_task(
    tasks: &AsyncComputeTaskPool,
    seed: u32,
    size: UVec2,
    generator: impl Fn(&mut CommandQueue, &mut Rng, UVec2) + Copy + Send + 'static,
) -> Task<FinishedMapGenerationTask> {
    tasks.spawn(async move {
        debug_assert_ne!(size.x, 0);
        debug_assert_ne!(size.y, 0);

        let task_pool = TaskPool::new();

        let fx = fragment(size.x);
        let fy = fragment(size.y);

        // Iterator that returns all fragments
        let iter = (0..=fx)
            .map(move |w| (0..=fy)
            .map(move |h| (w, h)))
            .flatten();

        // Tasks for all fragments
        let commands = task_pool.scope(|s| {
            for (fx, fy) in iter {
                s.spawn(async move {
                    // Create command queue
                    let mut commands = CommandQueue::default();
                    let mut random = fastrand::Rng::default();
                    random.seed(seed.into());

                    // Determine how many cells in this fragment should be generated
                    let (bx, ox) = defragment(fx, size.x);
                    let (by, oy) = defragment(fy, size.y);

                    // Run generation function on all cells
                    for cx in 0..=bx {
                        for cy in 0..=by {
                            let coordinates = UVec2 {
                                x: ox+cx,
                                y: oy+cy,
                            };
                            (generator)(&mut commands, &mut random, coordinates)
                        }
                    }

                    commands
                });
            }
        });

        // Return commands
        FinishedMapGenerationTask(commands.into())
    })
}

pub(super) fn single_continent(
    tasks: &AsyncComputeTaskPool,
    seed: u32,
    size: UVec2,
) -> Task<FinishedMapGenerationTask> {
    generic_continent_generation_task(tasks, seed, size, |x,y,z|{})
}