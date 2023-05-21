use std::collections::BTreeMap;
use bevy_ecs::{system::{CommandQueue, Spawn, Despawn}, prelude::Entity, world::Mut};
use eframe::egui;
use crate::{world::{sim::SimulationData, defs::species::{SpeciesBundle, Species}, thing::Name}, gui::EntityStringHashable};

const SUBTAB_KEY: &str = "edit_definitions_tab";

const SOFT_MAX_AGE: u32 = 10_000;
// This is the age at which a humanoid is considered an 'adult', and therefore can reproduce.
// DO NOT set this lower. No excuses.
const MIN_HUMANOID_AGE: u32 = 18;

pub(super) fn edit_definitions_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    if state.get(SUBTAB_KEY).is_none() { state.insert(SUBTAB_KEY.to_owned(), "Species".to_string()); }
    let current_tab = state.get_mut(SUBTAB_KEY).unwrap();

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
        "Species" => species_menu(ui, state, queue, sim),
        _ => todo!("Handle this case"),
    }
}

fn species_menu(
    ui: &mut egui::Ui,
    _state: &mut BTreeMap<String, String>,
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
                        maturity_age: 3,
                        max_age: 12,
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
                        maturity_age: 18,
                        max_age: 100,
                    },
                }
            )});
        }
    });

    ui.separator();

    let mut species_query = sim.world.query::<(Entity, &mut Name, &mut Species)>();

    egui::ScrollArea::both()
    .id_source("species_scroll_area")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for mut query_data in species_query.iter_mut(&mut sim.world) {
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
            let range = if species.humanoid { MIN_HUMANOID_AGE..=SOFT_MAX_AGE } else { u32::MIN..=SOFT_MAX_AGE };
            ui.add(egui::DragValue::new(&mut species.max_age).clamp_range(range));
            ui.end_row();

            let max_age = species.max_age.clone();

            // Age of maturity
            ui.label("Age of maturity");
            let range = if species.humanoid { MIN_HUMANOID_AGE..=max_age } else { u32::MIN..=max_age };
            ui.add(egui::Slider::new(&mut species.maturity_age, range));
            ui.end_row();
        });
    });
}