pub mod params;
pub mod time;
pub mod common;
pub mod people;
pub mod factions;

#[cfg(feature="graphics")]
mod graphics;

use bevy::prelude::*;
use params::{SimulationState, TimestepAmount, TimestepDirection};
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
    app.add_state::<TimestepDirection>();
    app.add_state::<TimestepAmount>();
    app.insert_resource(SimulationTime::default());

    // Run app
    app.run();
}