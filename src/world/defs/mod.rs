pub mod species;

use bevy_ecs::system::Resource;

/// Overarching information about the world.
#[derive(Resource, Debug)]
pub struct SimulationConfig {
    /// Prevents the user from changing any below values in the UI.
    pub locked_in: bool,

    pub name: String,
    pub seed: u32,
    pub direction: HistoryDirection,
    pub timespan: Timespan,
    pub increments_completed: u32,
    pub increments_for_completion: u32,
}

/// The direction history generates from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HistoryDirection {
    /// History generates forwards, as if time is progressing from a point in history.
    Forwards,
    /// History generates backwards, as if explaining the current state of the world.
    Backwards,
}

/// The timestep in which the simulator operates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Timespan {
    Months,
    Days,
}