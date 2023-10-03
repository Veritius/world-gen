use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use crate::map::{SimulationMap, generation::WorldGenerationMethod};

#[derive(Debug, Resource)]
pub struct MapConfigWindowOpen(pub bool);

pub fn map_config_window_system(
    mut ctxs: EguiContexts,
    mut window_open: ResMut<MapConfigWindowOpen>,
    mut map_config: ResMut<SimulationMap>,
) {
    if !window_open.0 { return; }

    egui::Window::new("World map parameters")
    .show(ctxs.ctx_mut(), |ui| {
        egui::Grid::new("map_parameters_grid")
        .striped(true)
        .show(ui, |ui| {
            // Random seed
            ui.label("Random seed");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut map_config.random_seed)
                    .custom_formatter(|n, _| {
                        let n = n as u64;
                        format!("{n:X}")
                    })
                    .custom_parser(|s| i64::from_str_radix(s, 16).map(|n| n as f64).ok()));
                if ui.button("New seed").clicked() {
                    map_config.random_seed = fastrand::u64(u64::MIN..=u64::MAX);
                }
            });
            ui.end_row();
            
            // World generator method selection
            ui.label("World generator");
            egui::ComboBox::new("map_world_gen", "")
            .selected_text(match map_config.gen_method {
                WorldGenerationMethod::SingleContinent => "Single Continent",
            })
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut map_config.gen_method, WorldGenerationMethod::SingleContinent, "Single Continent");
            });
            ui.end_row();

            // Map size
            ui.label("Map size");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut map_config.map_size.x).suffix('c'));
                ui.label("by");
                ui.add(egui::DragValue::new(&mut map_config.map_size.y).suffix('c'));
            });
            ui.end_row();
        });
    });
}