use bevy::prelude::App;
use crate::world::living::health_caching_system;

pub fn add_backward_day_presets(app: &mut App) {
    app.add_system(health_caching_system);
}