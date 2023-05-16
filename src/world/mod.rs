use rand::Rng;

pub mod event;
pub mod person;
pub mod place;
pub mod thing;

pub mod soft_limits {
    pub const MIN_YEARS_TO_SIMULATE: u32 = 50;
    pub const MAX_YEARS_TO_SIMULATE: u32 = 1000;
}

pub struct WorldPregenConfig {
    pub name: String,
    pub random_seed: u32,
    pub years_to_simulate: u32,
    pub chaos_multiplier: f32,
}

impl Default for WorldPregenConfig {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            random_seed: rand::thread_rng().gen::<u32>(),
            years_to_simulate: 100,
            chaos_multiplier: 1.0,
        }
    }
}