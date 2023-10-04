use bevy::{prelude::*, tasks::{AsyncComputeTaskPool, Task}};

use super::FinishedMapGenerationTask;

pub(super) fn single_continent(
    tasks: &AsyncComputeTaskPool,
    seed: u64,
    size: UVec2,
) -> Task<FinishedMapGenerationTask> {
    tasks.spawn(async move {
        let mut world = World::new();

        FinishedMapGenerationTask(world)
    })
}