use bevy::ecs::{system::{CommandQueue, Spawn, Despawn}, prelude::Entity, world::Mut};
use eframe::egui;
use crate::{world::{sim::SimulationData, defs::species::{SpeciesBundle, Species}, common::Name, time::TimeLength}, gui::{EntityStringHashable, AppMemory}};

use super::widgets::{time_length_drag_value, time_length_slider};

const SUBTAB_KEY: &str = "edit_definitions_tab";

const SOFT_MAX_AGE: TimeLength = TimeLength::from_years(10_000);
// This is the age at which a humanoid is considered an 'adult', and therefore can reproduce.
// DO NOT set this lower. No excuses.
const MIN_HUMANOID_AGE: TimeLength = TimeLength::from_years(18);

pub(super) fn edit_definitions_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    if memory.string_map.get(SUBTAB_KEY).is_none() { memory.string_map.insert(SUBTAB_KEY.to_owned(), "Species".to_string()); }
    let current_tab = memory.string_map.get_mut(SUBTAB_KEY).unwrap();

    // Tab change buttons
    egui::ScrollArea::horizontal()
    .id_source("defs_scroll_area")
    .auto_shrink([false, true])
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            ui.selectable_value(current_tab, "Species".to_owned(), "Species");
        });
    });
    
    ui.separator();
    
    // Tabs
    match current_tab.as_str() {
        "Species" => species_menu(ui, queue, sim),
        _ => todo!("Handle this case"),
    }
}

fn species_menu(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    ui.horizontal(|ui| {
        // New non-humanoid species (animals, etc)
        if ui.button("Add new creature").clicked() {
            queue.push(Spawn { bundle: (
                SpeciesBundle {
                    name: Name("New creature".to_owned()),
                    species: Species {
                        humanoid: false,
                        maturity_age: TimeLength::from_years(3),
                        max_age: TimeLength::from_years(12),
                    },
                }
            )});
        }

        // New humanoid species
        if ui.button("Add new humanoid").clicked() {
            queue.push(Spawn { bundle: (
                SpeciesBundle {
                    name: Name("New humanoid".to_owned()),
                    species: Species {
                        humanoid: true,
                        maturity_age: MIN_HUMANOID_AGE,
                        max_age: TimeLength::from_years(100),
                    },
                }
            )});
        }
    });

    ui.separator();

    let mut species_query = sim.app.world.query::<(Entity, &mut Name, &mut Species)>();

    egui::ScrollArea::both()
    .id_source("species_scroll_area")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for mut query_data in species_query.iter_mut(&mut sim.app.world) {
            species_editor(ui, queue, &mut query_data);
        }
    });
}

fn species_editor(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    query_data: &mut (Entity, Mut<Name>, Mut<Species>),
) {
    let (entity, name, species) = query_data;

    egui::CollapsingHeader::new(format!("{} ({:?})", &name.0, entity))
    .id_source(EntityStringHashable(*entity, "species_editor_section".to_owned()))
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Delete species").clicked() {
                queue.push(Despawn { entity: *entity });
            }
        });

        ui.add_space(3.0);

        // Species details
        egui::Grid::new(EntityStringHashable(*entity, "species_editor_details".to_owned()))
        .min_col_width(20.0)
        .spacing([15.0, 3.0])
        .striped(true)
        .show(ui, |ui| {
            // Species name
            ui.label("Name");
            ui.text_edit_singleline(&mut name.0);
            ui.end_row();

            // Humanoid
            ui.label("Humanoid");
            ui.checkbox(&mut species.humanoid, "Is this a humanoid species?");
            ui.end_row();

            // Max age
            ui.label("Max age");
            let range = if species.humanoid { MIN_HUMANOID_AGE..=SOFT_MAX_AGE } else { TimeLength::ZERO..=SOFT_MAX_AGE };
            ui.add(time_length_drag_value(&mut species.max_age).clamp_range(range));
            ui.end_row();

            let max_age = species.max_age.clone();

            // Age of maturity
            ui.label("Age of maturity");
            let range = if species.humanoid { MIN_HUMANOID_AGE..=max_age } else { TimeLength::ZERO..=max_age };
            ui.add(time_length_slider(&mut species.maturity_age, range));
            ui.end_row();
        });
    });
}