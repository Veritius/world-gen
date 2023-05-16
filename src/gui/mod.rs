use eframe::{egui, Frame, App};

pub struct WorldGenApp;

impl Default for WorldGenApp {
    fn default() -> Self {
        Self
    }
}

impl App for WorldGenApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Hello, world!");
        });
    }
}