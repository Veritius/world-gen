use bevy::prelude::App;
use crate::world::common::age_incrementor_system;
use crate::world::living::afflictions::affliction_progress_system;
use crate::world::living::health::{health_caching_system, death_system};

pub fn add_forward_day_presets(app: &mut App) {
    app.add_system(age_incrementor_system);
    app.add_system(health_caching_system);
    app.add_system(affliction_progress_system);
    app.add_system(death_system);
}