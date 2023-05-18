mod edit;
mod view;

use std::collections::BTreeMap;

use bevy_ecs::{prelude::Entity, world::World, schedule::Schedule};
use eframe::{egui, Frame, App};
use either::Either::{Right, Left};
use crate::world::sim::Simulation;

use self::view::view_ui;
use self::edit::edit_ui;

pub struct WorldGenApp {
    simulation: Simulation,
    state: BTreeMap<String, String>,
}

impl Default for WorldGenApp {
    fn default() -> Self {
        Self {
            simulation: Simulation::new(World::new(), Schedule::new()),
            state: BTreeMap::new(),
        }
    }
}

impl App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Show a different UI based on the simulation state
            match self.simulation.current() {
                Ok(value) => {
                    match value {
                        // The simulation is frozen and can be edited
                        Left(data) => edit_ui(ui, data),
                        // The simulation is running and the boundary can be read
                        Right(boundary) => view_ui(ui, boundary),
                    }
                },
                // The simulation boundary is poisoned
                // TODO: Handle this
                Err(_) => todo!(),
            }
        });
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Used for egui widget IDs to prevent collisions.
struct EntityStringHashable(pub Entity, pub String);