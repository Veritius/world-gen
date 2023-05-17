pub mod config;

use std::marker::PhantomData;

use bevy_ecs::{world::World, prelude::{Entity, Component}};
use eframe::{egui, Frame, App};
use self::config::{config_ui, ConfigState};

pub enum AppState {
    Config(ConfigState),
    Generating,
    Finished,
}

pub struct WorldGenApp {
    pub state: AppState,
}

impl Default for WorldGenApp {
    fn default() -> Self {
        Self {
            state: AppState::Config(ConfigState::default())
        }
    }
}

impl App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state {
                AppState::Config(ref mut cfg) => config_ui(ui, cfg),
                AppState::Generating => todo!(),
                AppState::Finished => todo!(),
            }
        });
    }
}

/// Like Bevy's CommandQueue but bad.
struct KnockoffCommandQueue(Vec<Box<dyn QueueAction>>);

impl KnockoffCommandQueue {
    pub fn push(&mut self, action: Box<dyn QueueAction>) {
        self.0.push(action);
    }

    pub fn execute(&mut self, world: &mut World) {
        for action in &self.0 {
            action.perform(world);
        }

        self.0.clear();
    }
}

trait QueueAction {
    fn perform(&self, world: &mut World);
}

struct DeleteEntity(pub Entity);

impl QueueAction for DeleteEntity {
    fn perform(&self, world: &mut World) {
        world.despawn(self.0);
    }
}

struct InsertComponent<T: Component> {
    pub entity: Entity,
    pub component: T,
}

impl<T: Component + Clone> QueueAction for InsertComponent<T> {
    fn perform(&self, world: &mut World) {
        let Some(mut entity) = world.get_entity_mut(self.entity) else { return };
        entity.insert(self.component.clone());
    }
}

struct RemoveComponent<T: Component>(pub Entity, pub PhantomData<T>);

impl<T: Component> QueueAction for RemoveComponent<T> {
    fn perform(&self, world: &mut World) {
        let Some(mut entity) = world.get_entity_mut(self.0) else { return };
        entity.remove::<T>();
    }
}