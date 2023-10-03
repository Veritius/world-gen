//! Simulation parameters and state information.

use bevy::prelude::*;

#[derive(Debug, Clone, Default, Hash, Reflect, States, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimulationState {
    #[default]
    Setup,
    Paused,
    Running,
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