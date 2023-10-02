use bevy::prelude::*;

#[derive(Debug, Clone, Default, Hash, Reflect, States, PartialEq, Eq, PartialOrd, Ord)]
pub enum SimulationState {
    #[default]
    Paused,
    Running,
}