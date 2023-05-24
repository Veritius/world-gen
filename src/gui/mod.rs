mod edit;
mod view;
mod ecs;
mod notifs;

use std::collections::BTreeMap;
use bevy::ecs::system::CommandQueue;
use bevy::ecs::prelude::Entity;
use bevy::utils::HashSet;
use eframe::{egui, Frame, App};
use either::Either::{Right, Left};
use replace_with::replace_with_or_abort;
use crate::world::defs::{SimulationConfig, Timespan, HistoryDirection};
use crate::world::presets::bck_day::add_backward_day_presets;
use crate::world::presets::bck_mon::add_backward_month_presets;
use crate::world::presets::fwd_day::add_forward_day_presets;
use crate::world::presets::fwd_mon::add_forward_month_presets;
use crate::world::sim::{Simulation, validate_world, SimulationData};

use self::notifs::{Notification, show_notifications};
use self::view::view_ui;
use self::edit::edit_ui;

pub struct WorldGenApp {
    simulation: Simulation,
    memory: AppMemory,
}

/// Used to store things across frames.
struct AppMemory {
    markers: HashSet<String>,
    string_map: BTreeMap<String, String>,
    notifications: Vec<Notification>,
}

impl Default for AppMemory {
    fn default() -> Self {
        Self {
            markers: HashSet::new(),
            string_map: BTreeMap::new(),
            notifications: vec![],
        }
    }
}

impl Default for WorldGenApp {
    fn default() -> Self {
        Self {
            simulation: Simulation::default(),
            memory: AppMemory::default(),
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
                        Left(data) => {
                            let mut queue = CommandQueue::default();
                            edit_ui(ui, &mut self.memory, &mut queue, data);
                            queue.apply(&mut data.app.world);
                        },
                        // The simulation is running and the boundary can be read
                        Right(boundary) => {
                            // Always repaint the UI while the simulation is in progress
                            if boundary.steps_complete != boundary.steps_total { ctx.request_repaint(); }

                            view_ui(ui, &mut self.memory, boundary);
                        },
                    }
                },
                // The simulation boundary is poisoned
                // TODO: Handle this
                Err(_) => todo!(),
            }
        });

        show_notifications(&mut self.memory.notifications, ctx);

        // Start simulation
        if self.memory.markers.contains("try_execute_simulation") {
            self.memory.markers.remove("try_execute_simulation");

            match self.simulation.current_or_err() {
                Ok(simulation) => {
                    systems_check(simulation);
                },
                Err(_error) => todo!(),
            }

            replace_with_or_abort(&mut self.simulation, |sim| sim.try_execute().0);
        }

        // Freeze simulation
        if self.memory.markers.contains("try_freeze_simulation") {
            self.memory.markers.remove("try_freeze_simulation");
            replace_with_or_abort(&mut self.simulation, |sim| sim.freeze().unwrap());
        }
    }
}

// Adds the necessary systems for simulation
fn systems_check(
    simulation: &mut SimulationData,
) {
    // Get direction and timestep
    let mut cfg = simulation.app.world.resource_mut::<SimulationConfig>();
    if cfg.locked_in { return; } // Preset is already set
    let (direction, timestep) = (cfg.direction.clone(), cfg.timespan.clone());
    cfg.locked_in = true;
    drop(cfg);

    // Apply systems to app
    match (direction, timestep) {
        (HistoryDirection::Forwards, Timespan::Months) => add_forward_month_presets(&mut simulation.app),
        (HistoryDirection::Forwards, Timespan::Days) => add_forward_day_presets(&mut simulation.app),
        (HistoryDirection::Backwards, Timespan::Months) => add_backward_month_presets(&mut simulation.app),
        (HistoryDirection::Backwards, Timespan::Days) => add_backward_day_presets(&mut simulation.app),
    }

    // Validate world to make sure everything is in order
    validate_world(&mut simulation.app.world);
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Used for egui widget IDs to prevent collisions.
struct EntityStringHashable(pub Entity, pub String);