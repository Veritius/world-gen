use bevy::{prelude::*, ecs::system::{CommandQueue, Spawn, SystemState, Despawn}};
use eframe::egui;
use crate::{world::{sim::SimulationData, map::tile::{MapTileDefBundle, MapTileDefinition}, common::Name}, gui::EntityStringHashable};

pub(super) fn tiles_menu(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    // New humanoid species
    if ui.button("New tile type").clicked() {
        queue.push(Spawn {
            bundle: MapTileDefBundle {
                name: "A new tile".into(),
                def: MapTileDefinition::default(),
            }
        });
    }

    ui.separator();

    let mut state: SystemState<Query<(Entity, &mut Name, &mut MapTileDefinition)>> = SystemState::new(&mut sim.app.world);
    let mut state_mut = state.get_mut(&mut sim.app.world);

    egui::ScrollArea::both()
    .id_source("species_scroll_area")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for query_data in state_mut.iter_mut() {
            tile_editor(ui, queue, query_data);
        }
    });
}

fn tile_editor(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    query_data: (Entity, Mut<Name>, Mut<MapTileDefinition>)
) {
    let (entity, mut name, mut tile) = query_data;

    egui::CollapsingHeader::new(format!("{} ({:?})", &name.0, entity))
    .id_source(EntityStringHashable(entity, "tile_editor_section".to_owned()))
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Delete tile type").clicked() {
                queue.push(Despawn { entity });
            }
        });

        ui.add_space(3.0);
        
        egui::Grid::new(EntityStringHashable(entity, "tile_editor_details".to_owned()))
        .min_col_width(20.0)
        .spacing([15.0, 3.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Name");
            ui.add_sized([200.0, 18.0], |ui: &mut egui::Ui| { ui.text_edit_singleline(&mut name.0) });
            ui.end_row();

            ui.label("Passable by");
            ui.vertical(|ui| {
                ui.checkbox(&mut tile.passable_by_land, "land");
                ui.checkbox(&mut tile.passable_by_air, "air");
                ui.checkbox(&mut tile.passable_by_water, "water");
            });
            ui.end_row();
        });
    });
}