use std::{sync::{RwLock, Arc, RwLockReadGuard}, thread::{JoinHandle, self}};
use bevy_ecs::{world::World, schedule::Schedule, system::Resource};
use either::Either::{self, Left, Right};

pub type Boundary = Arc<RwLock<SimulationBoundary>>;

/// The simulation of the world.
pub struct Simulation {
    state: SimulationState,
}

impl Simulation {
    /// Creates a new frozen simulation from a `World` and `Schedule`.
    pub fn new(world: World, schedule: Schedule) -> Self {
        Self {
            state: SimulationState::Frozen(
                SimulationData {
                    schedule,
                    world,
                }
            )
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

    /// Begins executing the simulation. Returns the Boundary if successful, and sets `self` to the `Executing` variant.
    /// Returns a `SimulationError` if this simulation is already running.
    pub fn try_execute(mut self) -> Result<Boundary, SimulationError> {
        // Check this Simulation is frozen
        let (schedule, world) =
        if let SimulationState::Frozen(simulation_internal) = self.state {
            (simulation_internal.schedule, simulation_internal.world)
        } else {
            return Err(SimulationError::AlreadySimulating);
        };

        // Create boundary object
        let status = Arc::new(RwLock::new(SimulationBoundary::default()));
        let status_for_thread = status.clone();
        
        // Create execution thread
        let thread: JoinHandle<SimulationData> = thread::spawn(move || {
            // Take ownership
            let status = status_for_thread;
            let (mut schedule, mut world) = (schedule, world);

            // Logic loop
            loop {
                if status.read().unwrap().stop_next_tick {
                    return SimulationData { schedule, world };
                }

                schedule.run(&mut world);
            }
        });

        // Change simulation state
        self.state = SimulationState::Executing {
            boundary: status.clone(),
            thread,
        };

        // Return boundary
        Ok(status)
    }

    /// Signals the simulation to stop and blocks until it finishes.
    /// Returns a `SimulationError` if the boundary is poisoned or the simulation panicked.
    pub fn freeze(mut self) -> Result<(), SimulationError> {
        // Take ownership of self.state
        let state = self.state;

        // Check state is executing and not frozen
        let (boundary, thread) =
        if let SimulationState::Executing { boundary, thread } = state {
            (boundary, thread)
        } else {
            // Already frozen
            return Ok(());
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
            return Ok(());
        } else {
            return Err(SimulationError::SimulationPanicked);
        };
    }

    pub fn world(&mut self) -> Result<&mut World, SimulationError> {
        match &mut self.state {
            SimulationState::Frozen(ref mut data) => { return Ok(&mut data.world) },
            SimulationState::Executing { boundary: _, thread: _ } => { return Err(SimulationError::NotFrozen) },
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

pub struct SimulationData {
    pub schedule: Schedule,
    pub world: World,
}

/// Allows communication between the GUI and the simulation thread.
pub struct SimulationBoundary {
    /// If `true`, the simulation will freeze on the next tick.
    pub stop_next_tick: bool,

    pub steps_complete: u32,
    pub steps_total: u32,
}

impl Default for SimulationBoundary {
    fn default() -> Self {
        Self {
            stop_next_tick: false,
            steps_complete: 0,
            steps_total: 0,
        }
    }
}

#[derive(Resource)]
pub struct SimulationComplete;