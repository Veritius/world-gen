use bevy::prelude::World;

pub trait WorldGenerator {
    /// Starts the generation.
    fn execute(&mut self);
    /// Checks if generation is finished.
    fn is_finished(&self) -> bool;
    /// Blocks the current thread, and waits until it's finished, then applies the changes.
    fn apply(&mut self, world: &mut World);
}