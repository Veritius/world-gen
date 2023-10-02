pub mod state;
pub mod common;
pub mod people;
pub mod factions;

#[cfg(feature="graphics")]
mod graphics;

use bevy::prelude::*;
use common::SimulationTime;
use state::SimulationState;

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
    app.insert_resource(SimulationTime::default());

    // Run app
    app.run();
}