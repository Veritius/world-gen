//! Simulation parameters and state information.

use bevy::prelude::*;
use fastrand::Rng;

#[derive(Debug, Clone, Default, Hash, Reflect, States, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimulationState {
    #[default]
    Setup,
    Paused,
    Running,
}

#[derive(Debug, Clone, Resource)]
pub struct SimulationRandom {
    pub seed: u64,
    pub random_state: Rng,
}

impl Default for SimulationRandom {
    fn default() -> Self {
        let seed = fastrand::u64(u64::MIN..=u64::MAX);
        Self {
            seed,
            random_state: fastrand::Rng::with_seed(seed),
        }
    }
}

#[derive(Debug, Clone, Default, Reflect, Resource, PartialEq, Eq)]
pub enum TimestepDirection {
    #[default]
    Forwards,
    Backwards,
}

#[derive(Debug, Clone, Default, Reflect, Resource, PartialEq, Eq)]
pub enum TimestepAmount {
    #[default]
    Days,
    Months,
}