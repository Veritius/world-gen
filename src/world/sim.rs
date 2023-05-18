use std::{sync::{RwLock, Arc}, thread::{JoinHandle, self}};
use bevy_ecs::{world::World, schedule::Schedule, system::Resource};

pub enum Simulation {
    Unstarted,
    Started {
        status: Arc<RwLock<SimulationStatus>>,
        handle: JoinHandle<World>,
    }
}

impl Simulation {
    pub fn start_simulation(mut self, mut world: World, mut schedule: Schedule) -> Result<Arc<RwLock<SimulationStatus>>, ()> {
        match self {
            Simulation::Unstarted => {
                let status = Arc::new(RwLock::new(SimulationStatus::default()));

                // Clone so the value isn't moved out when it's needed later
                let status_for_thread = status.clone();

                let thread = thread::spawn(move || {
                    let status = status_for_thread;

                    loop {
                        schedule.run(&mut world);

                        // Marker resource that stops the execution
                        if world.get_resource::<SimulationComplete>().is_some() { break; }
                    }

                    world
                });

                self = Simulation::Started { status: status.clone(), handle: thread };
                Ok(status)
            },
            Simulation::Started { status: _, handle: _ } => { Err(()) },
        }
    }
}

pub struct SimulationStatus {
    pub steps_complete: u32,
}

impl Default for SimulationStatus {
    fn default() -> Self {
        Self {
            steps_complete: 0
        }
    }
}

#[derive(Resource)]
pub struct SimulationComplete;