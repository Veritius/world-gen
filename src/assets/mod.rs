//! Asset types loaded by world-gen.

pub mod wordsets;

use bevy::prelude::*;
use bevy_common_assets::yaml::YamlAssetPlugin;

use self::wordsets::*;

pub fn setup_assets_for_app(app: &mut App) {
    // Add asset types
    app.add_plugins(YamlAssetPlugin::<WordSet>::new(&["wordset.yml"]));

    // Load data on startup
    app.init_resource::<LoadDataFolderImmediately>();
    app.add_systems(Startup, load_data_folder_on_startup_system);
}

/// If this resource is present in the `Startup` stage, all assets in the `data` folder will be loaded and handles will be kept in memory.
#[derive(Default, Resource)]
pub struct LoadDataFolderImmediately;

fn load_data_folder_on_startup_system(
    asset_server: Res<AssetServer>,
    marker: Option<Res<LoadDataFolderImmediately>>,
    mut handles: Local<Vec<HandleUntyped>>,
) {
    if marker.is_none() { return; }
    *handles = asset_server.load_folder("data")
        // TODO: Error tolerance
        .expect("Failed while loading items in folder");
    
}