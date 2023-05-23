//! Helpers for UI stuff working with Bevy and its ECS.

use bevy::{prelude::*, ecs::system::Command};

/// Spawns an entity as a child of another. If the parent no longer exists, it doesn't set a parent.
pub struct SpawnChild<T> {
    pub parent: Entity,
    pub bundle: T,
}

impl<T: Bundle> Command for SpawnChild<T> {
    fn write(self, world: &mut World) {
        let entity = world.spawn(self.bundle).id();
        let action = AddChild { parent: self.parent, child: entity };
        action.write(world);
    }
}