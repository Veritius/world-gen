use bevy::prelude::App;
use crate::world::common::age_incrementor_system;
use crate::world::living::health_caching_system;
use crate::world::living::afflictions::affliction_progress_system;

pub fn add_forward_month_presets(app: &mut App) {
    app.add_system(age_incrementor_system);
    app.add_system(health_caching_system);
    app.add_system(affliction_progress_system);
}