pub mod config;

use eframe::{egui, Frame, App};
use crate::world::WorldPregenConfig;

use self::config::{config_ui, Tab, ConfigState};

pub enum AppState {
    Config(ConfigState),
    Generating,
    Finished,
}

pub struct WorldGenApp {
    pub state: AppState,
}

impl Default for WorldGenApp {
    fn default() -> Self {
        Self {
            state: AppState::Config(ConfigState::default())
        }
    }
}

impl App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match &mut self.state {
                AppState::Config(ref mut cfg) => config_ui(ui, cfg),
                AppState::Generating => todo!(),
                AppState::Finished => todo!(),
            }
        });
    }
}