use std::{sync::{RwLock, Arc, RwLockReadGuard}, thread::{JoinHandle, self}, time::Instant};
use bevy::{ecs::{world::World, system::Resource, prelude::Entity, query::With}, prelude::{App, HierarchyPlugin, Or}};
use either::Either::{self, Left, Right};
use crate::world::{defs::SimulationConfig, person::Person, place::{Region, Settlement}};
use super::defs::{HistoryDirection, Timespan};

pub const MIN_SIM_STEPS: u32 = 10;
pub const RECORD_LENGTH: usize = 250;

pub type Boundary = Arc<RwLock<SimulationBoundary>>;

/// The simulation of the world.
pub struct Simulation {
    state: SimulationState,
}

impl Simulation {
    /// Creates a new frozen simulation from a Bevy `App`
    pub fn new(app: App) -> Self {
        Self {
            state: SimulationState::Frozen(SimulationData { app })
        }
    }

    /// Returns a mutable reference to the `SimulationData` or an error if it's not possible.
    pub fn current_or_err(&mut self) -> Result<&mut SimulationData, SimulationError> {
        match self.current() {
            Ok(value) => {
                match value {
                    Left(world) => { return Ok(world); },
                    Right(_) => { return Err(SimulationError::NotFrozen); },
                }
            },
            Err(error) => { return Err(error); }
        }
    }

    /// Returns either a mutable reference to the `SimulationData` if the simulation is frozen, a `RwLockReadGuard<SimulationBoundary>` if executing, or a `SimulationError` if the boundary is poisoned.
    pub fn current(&mut self) -> Result<Either<&mut SimulationData, RwLockReadGuard<SimulationBoundary>>, SimulationError> {
        match &mut self.state {
            SimulationState::Frozen(ref mut data) => {
                return Ok(Left(data))
            },
            SimulationState::Executing { boundary, thread: _ } => {
                match boundary.read() {
                    Ok(boundary) => {
                        return Ok(Right(boundary))
                    },
                    Err(_) => {
                        return Err(SimulationError::BoundaryPoisoned)
                    }
                }
            },
        }
    }

    /// Begins executing the simulation. Always returns the simulation, and returns the Boundary if successful.
    /// Returns a `SimulationError` if this simulation is already running.
    pub fn try_execute(mut self) -> (Self, Result<Boundary, SimulationError>) {
        let app =
        if let SimulationState::Frozen(simulation_internal) = self.state {
            simulation_internal.app
        } else {
            return (self, Err(SimulationError::AlreadySimulating));
        };

        // Create boundary object
        let status = Arc::new(RwLock::new(SimulationBoundary::default()));
        let status_for_thread = status.clone();
        
        // Create execution thread
        let thread: JoinHandle<SimulationData> = thread::spawn(move || {
            // Take ownership
            let status = status_for_thread;
            let mut app = app;

            // Logic loop
            loop {
                let cfg = app.world.resource::<SimulationConfig>();

                // Check we aren't due to stop
                if status.read().unwrap().stop_next_tick || (cfg.increments_completed == cfg.increments_for_completion) {
                    return SimulationData { app };
                }

                // Time before the tick happens
                let now = Instant::now();

                // For testing
                // thread::sleep(std::time::Duration::from_secs_f32(0.1));

                // Run one tick
                app.update();

                // Measure how long it took to tick
                let elapsed = now.elapsed();

                // Increase increment counter by 1
                let mut cfg = app.world.resource_mut::<SimulationConfig>();
                cfg.increments_completed += 1;

                // Write to boundary
                let mut status = status.write().unwrap();
                status.steps_total = cfg.increments_for_completion;
                status.steps_complete = cfg.increments_completed;

                fn push_to_cap<T>(
                    cap: usize,
                    value: &mut Vec<T>,
                    push: T,
                ) {
                    let vlen = value.len();
                    if vlen == cap {
                        value.remove(0);
                    }
                    value.push(push);
                }

                push_to_cap::<f64>(RECORD_LENGTH, &mut status.tick_time_history, elapsed.as_secs_f64());
                push_to_cap::<f64>(RECORD_LENGTH, &mut status.entity_count_history, app.world.query::<Entity>().iter(&app.world).len() as f64);
                push_to_cap::<f64>(RECORD_LENGTH, &mut status.people_count_history, app.world.query_filtered::<Entity, With<Person>>().iter(&app.world).len() as f64);
                push_to_cap::<f64>(RECORD_LENGTH, &mut status.place_count_history, app.world.query_filtered::<Entity, Or<(With<Region>, With<Settlement>)>>().iter(&app.world).len() as f64)
            }
        });

        // Change simulation state
        self.state = SimulationState::Executing {
            boundary: status.clone(),
            thread,
        };

        // Return boundary
        (self, Ok(status))
    }

    /// Signals the simulation to stop and blocks until it finishes.
    /// Returns the frozen simulation if successful, and returns a `SimulationError` if the boundary is poisoned or the simulation panicked.
    pub fn freeze(mut self) -> Result<Self, SimulationError> {
        // Check state is executing and not frozen
        let (boundary, thread) =
        if let SimulationState::Executing { boundary, thread } = self.state {
            (boundary, thread)
        } else {
            // Already frozen
            return Ok(self);
        };

        // Signal simulation to freeze
        if let Ok(mut boundary) = boundary.write() {
            boundary.stop_next_tick = true;
        } else {
            return Err(SimulationError::BoundaryPoisoned);
        }

        // Wait for thread
        if let Ok(data) = thread.join() {
            self.state = SimulationState::Frozen(data);
            // Success!
            return Ok(self);
        } else {
            return Err(SimulationError::SimulationPanicked);
        };
    }

    pub fn world(&mut self) -> Result<&mut World, SimulationError> {
        match &mut self.state {
            SimulationState::Frozen(ref mut data) => { return Ok(&mut data.app.world) },
            SimulationState::Executing { boundary: _, thread: _ } => { return Err(SimulationError::NotFrozen) },
        }
    }
}

impl Default for Simulation {
    fn default() -> Self {
        let mut app = App::new();

        app.add_plugin(HierarchyPlugin);

        app.insert_resource(SimulationConfig {
            locked_in: false,
            name: "".to_string(),
            seed: rand::random(),
            direction: HistoryDirection::Forwards,
            timespan: Timespan::Months,
            increments_completed: 0,
            increments_for_completion: MIN_SIM_STEPS,
        });

        Self {
            state: SimulationState::Frozen(SimulationData { app })
        }
    }
}

pub enum SimulationState {
    /// The simulation is frozen, and mutably accessible.
    Frozen(SimulationData),

    /// The simulation is executing on another thread, and must be communicated with using the boundary.
    Executing {
        boundary: Boundary,
        thread: JoinHandle<SimulationData>,
    },
}

/// Error type for operations on the `Simulation` object.
#[derive(Debug)]
pub enum SimulationError {
    /// The communications boundary was poisoned.
    BoundaryPoisoned,
    
    /// The simulation thread panicked somewhere.
    SimulationPanicked,

    /// The simulation is already running.
    AlreadySimulating,

    /// The simulation isn't frozen when it should be.
    NotFrozen,
}

pub struct SimulationData{
    pub app: App
}

#[derive(Debug)]
/// Allows communication between the GUI and the simulation thread.
pub struct SimulationBoundary {
    /// If `true`, the simulation will freeze on the next tick.
    pub stop_next_tick: bool,

    // Completion measurement
    pub steps_complete: u32,
    pub steps_total: u32,

    // Simulation statistics
    pub tick_time_history: Vec<f64>,
    pub entity_count_history: Vec<f64>,
    pub people_count_history: Vec<f64>,
    pub place_count_history: Vec<f64>,
}

impl Default for SimulationBoundary {
    fn default() -> Self {
        Self {
            stop_next_tick: false,

            steps_complete: 0,
            steps_total: 0,

            tick_time_history: Vec::with_capacity(RECORD_LENGTH),
            entity_count_history: Vec::with_capacity(RECORD_LENGTH),
            people_count_history: Vec::with_capacity(RECORD_LENGTH),
            place_count_history: Vec::with_capacity(RECORD_LENGTH),
        }
    }
}

#[derive(Resource)]
pub struct SimulationComplete;

/// Checks over everything in the world and ensures it's all working well.
pub fn validate_world(world: &mut World) {

}