mod edit;
mod view;
mod ecs;
mod notifs;
mod modal;
mod sim;

use std::collections::{BTreeMap, BTreeSet};
use bevy::ecs::system::CommandQueue;
use bevy::ecs::prelude::Entity;
use eframe::{egui, Frame, App};
use either::Either::{Right, Left};
use crate::world::defs::{SimulationConfig, Timespan, HistoryDirection};
use crate::world::presets::bck_day::add_backward_day_presets;
use crate::world::presets::bck_mon::add_backward_month_presets;
use crate::world::presets::fwd_day::add_forward_day_presets;
use crate::world::presets::fwd_mon::add_forward_month_presets;
use crate::world::sim::{Simulation, validate_world, SimulationData};

use self::modal::ModalWindow;
use self::notifs::{Notification, show_notifications, update_notifications};
use self::sim::simulation_fns;
use self::view::view_ui;
use self::edit::edit_ui;

pub struct WorldGenApp {
    simulation: Simulation,
    memory: AppMemory,
}

/// Used to store things across frames.
struct AppMemory {
    markers: BTreeSet<String>,
    string_map: BTreeMap<String, String>,
    modal_popup: Option<ModalWindow>,
    notifications: Vec<Notification>,
}

impl Default for AppMemory {
    fn default() -> Self {
        Self {
            markers: BTreeSet::new(),
            string_map: BTreeMap::new(),
            modal_popup: None,
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
        update_notifications(&mut self.memory.notifications, ctx);
        show_notifications(&self.memory.notifications, ctx);

        // Show a different UI based on the simulation state
        match self.simulation.current() {
            Ok(value) => {
                match value {
                    // The simulation is frozen and can be edited
                    Left(data) => {
                        egui::CentralPanel::default()
                        .show(ctx, |ui| {
                            let mut queue = CommandQueue::default();
                            edit_ui(ui, &mut self.memory, &mut queue, data);
                            queue.apply(&mut data.app.world);
                        });
                    },
                    // The simulation is running and the boundary can be read
                    Right(boundary) => {
                        // Always repaint the UI while the simulation is in progress
                        if boundary.steps_complete != boundary.steps_total { ctx.request_repaint(); }

                        if boundary.simulation_exited { self.memory.markers.insert("try_freeze_simulation".to_string()); }

                        egui::TopBottomPanel::top("sim_status_panel")
                        .show_separator_line(false)
                        .show(ctx, |ui| {
                            ui.add_space(2.0);
                            ui.horizontal(|ui| {
                                if ui.button("Stop simulation").clicked() {
                                    self.memory.markers.insert("try_freeze_simulation".to_owned());
                                }
                        
                                let percent = boundary.steps_complete as f32 / boundary.steps_total as f32;
                                ui.add(egui::ProgressBar::new(percent).show_percentage());
                            });
                            ui.add_space(1.0);
                        });

                        egui::CentralPanel::default().show(ctx, |ui| {
                            view_ui(ui, &mut self.memory, boundary);
                        });
                    },
                }
            },
            // The simulation boundary is poisoned
            Err(_) => {
                self.memory.markers.insert("try_freeze_simulation".to_owned());
            },
        }
        
        // Modal windows
        if let Some(popup) = &self.memory.modal_popup {
            popup.display(ctx, &mut self.memory.markers);
        }
        
        if self.memory.markers.contains("remove_modal") {
            self.memory.markers.remove("remove_modal");
            self.memory.modal_popup = None;
        }

        simulation_fns(self);
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