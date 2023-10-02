use bevy::prelude::*;

fn main() {
    // Bevy plugins
    // Affected by feature flags
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    // Graphics plugins
    #[cfg(feature="graphics")]
    app.add_plugins(bevy_egui::EguiPlugin);

    // Run app
    app.run();
}