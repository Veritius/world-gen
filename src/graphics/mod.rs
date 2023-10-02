mod editing;
mod menubar;
mod pause;
mod params;

use bevy::prelude::*;
use bevy_pancam::PanCam;

use menubar::menu_bar_system;
use pause::pause_menu_system;

use self::{editing::{person::{person_editing_system, person_listing_system, PersonListWindowOpen}, factions::{FactionListWindowOpen, faction_listing_system, faction_editing_system}}, params::{SimulationSettingsWindowOpen, simulation_parameters_settings_window_system}};

/// Graphics functionality
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        // Basic UI
        app.add_systems(Startup, create_camera_system);
        app.add_systems(Update, pause_menu_system);
        app.add_systems(Update, menu_bar_system);

        // Listing systems
        app.insert_resource(PersonListWindowOpen(false));
        app.add_systems(Update, person_listing_system);
        app.insert_resource(FactionListWindowOpen(false));
        app.add_systems(Update, faction_listing_system);

        // Editing systems
        app.insert_resource(SimulationSettingsWindowOpen(true));
        app.add_systems(Update, simulation_parameters_settings_window_system);
        app.add_systems(Update, person_editing_system);
        app.add_systems(Update, faction_editing_system);
    }
}

/// Creates a camera for observing the world
fn create_camera_system(
    mut commands: Commands,
) {
    let mut pancam = PanCam::default();
    pancam.grab_buttons = vec![MouseButton::Middle];
    commands.spawn((Camera2dBundle::default(), pancam));
}