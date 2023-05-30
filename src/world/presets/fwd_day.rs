use bevy::prelude::App;
use crate::world::common::age_incrementor_system;

pub fn add_forward_day_presets(app: &mut App) {
    app.add_system(age_incrementor_system);
}