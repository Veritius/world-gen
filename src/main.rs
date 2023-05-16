pub mod gui;
pub mod world;

use eframe::egui;
use gui::WorldGenApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(600.0, 400.0)),
        ..Default::default()
    };
    eframe::run_native(
        "World Generator",
        options,
        Box::new(|_cc| Box::<WorldGenApp>::default()),
    )
}