mod pause;

use bevy::prelude::*;
use bevy_pancam::PanCam;

use self::pause::pause_menu_system;

/// Graphics functionality
pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_camera_system);
        app.add_systems(Update, pause_menu_system);
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