mod edit;
mod view;

use std::collections::BTreeMap;
use bevy_ecs::system::CommandQueue;
use bevy_ecs::prelude::Entity;
use eframe::{egui, Frame, App};
use either::Either::{Right, Left};
use replace_with::replace_with_or_abort;
use crate::world::defs::{SimulationConfig, Timespan, HistoryDirection};
use crate::world::schedules::bck_day::backwards_days;
use crate::world::schedules::bck_mon::backwards_months;
use crate::world::schedules::fwd_day::forwards_days;
use crate::world::schedules::fwd_mon::forwards_months;
use crate::world::sim::{Simulation, validate_world};

use self::view::view_ui;
use self::edit::edit_ui;

pub struct WorldGenApp {
    simulation: Simulation,
    state: BTreeMap<String, String>,
}

impl Default for WorldGenApp {
    fn default() -> Self {
        Self {
            simulation: Simulation::default(),
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
                        Left(data) => {
                            let mut queue = CommandQueue::default();
                            edit_ui(ui, &mut self.state, &mut queue, data);
                            queue.apply(&mut data.world);
                        },
                        // The simulation is running and the boundary can be read
                        Right(boundary) => {
                            view_ui(ui, &mut self.state, boundary);

                            // Always repaint the UI in the view state, as there are widgets that change without user input
                            ctx.request_repaint();
                        },
                    }
                },
                // The simulation boundary is poisoned
                // TODO: Handle this
                Err(_) => todo!(),
            }
        });

        // Start simulation
        if self.state.contains_key("try_execute_simulation") {
            self.state.remove("try_execute_simulation");

            match self.simulation.current_or_err() {
                Ok(simulation) => {
                    // Get direction and timestep
                    let mut cfg = simulation.world.resource_mut::<SimulationConfig>();
                    if cfg.locked_in { return; } // Schedule is already set
                    let (direction, timestep) = (cfg.direction.clone(), cfg.timespan.clone());
                    cfg.locked_in = true;
                    drop(cfg);

                    // Apply systems to schedule object
                    match (direction, timestep) {
                        (HistoryDirection::Forwards, Timespan::Months) => forwards_months(&mut simulation.schedule),
                        (HistoryDirection::Forwards, Timespan::Days) => forwards_days(&mut simulation.schedule),
                        (HistoryDirection::Backwards, Timespan::Months) => backwards_months(&mut simulation.schedule),
                        (HistoryDirection::Backwards, Timespan::Days) => backwards_days(&mut simulation.schedule),
                    }

                    // Initialise schedule
                    simulation.schedule.initialize(&mut simulation.world).expect("Schedule failed to build");

                    // Validate world to make sure everything is in order
                    validate_world(&mut simulation.world);
                },
                Err(_error) => todo!(),
            }

            replace_with_or_abort(&mut self.simulation, |sim| sim.try_execute().0);
        }

        // Freeze simulation
        if self.state.contains_key("try_freeze_simulation") {
            self.state.remove("try_freeze_simulation");
            replace_with_or_abort(&mut self.simulation, |sim| sim.freeze().unwrap());
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
/// Used for egui widget IDs to prevent collisions.
struct EntityStringHashable(pub Entity, pub String);