pub mod state;
pub mod common;
pub mod people;

#[cfg(feature="graphics")]
mod graphics;

use bevy::prelude::*;
use state::SimulationState;

fn main() {
    // Bevy plugins
    // Affected by feature flags
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Add states
    app.add_state::<SimulationState>();

    // Graphics plugins
    #[cfg(feature="graphics")] {
        app.add_plugins(bevy_egui::EguiPlugin);
        app.add_plugins(bevy_pancam::PanCamPlugin);
        app.add_plugins(graphics::GraphicsPlugin);
    }

    // Run app
    app.run();
}