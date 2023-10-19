pub(in super::super) mod initial;

mod terrain;

use std::sync::{Arc, Mutex};
use bevy::{prelude::*, tasks::Task, ecs::system::CommandQueue};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub enum WorldGenerationMethod {
    #[default]
    SingleContinent,
}

/// Raise to clear all map cells and regenerate the world.
#[derive(Debug, Event)]
pub struct RegenerateMapEvent;

#[derive(Resource)]
pub struct RunningMapGenerationTask {
    /// Percent completed, from 0.0 to 1.0.
    /// Intended to be quickly mutated by the processing thread, and read by the UI.
    pub completion: Arc<Mutex<(f32, String)>>,
    task: Task<FinishedMapGenerationTask>,
}

struct FinishedMapGenerationTask(pub Box<[CommandQueue]>);