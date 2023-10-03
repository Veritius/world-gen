pub mod params;
pub mod time;
pub mod common;
pub mod people;
pub mod factions;
pub mod species;

#[cfg(feature="graphics")]
mod graphics;

use bevy::prelude::*;
use params::{SimulationState, TimestepAmount, TimestepDirection, SimulationRandom};
use time::SimulationTime;

fn main() {
    // Bevy plugins
    // Affected by feature flags
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Graphics plugins
    #[cfg(feature="graphics")] {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_plugins(bevy_pancam::PanCamPlugin);
        app.add_plugins(graphics::GraphicsPlugin);
    }

    // Simulation data
    app.add_state::<SimulationState>();
    app.init_resource::<SimulationTime>();
    app.init_resource::<SimulationRandom>();
    app.init_resource::<TimestepDirection>();
    app.init_resource::<TimestepAmount>();

    // Run app
    app.run();
}