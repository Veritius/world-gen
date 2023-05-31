use std::{collections::{BTreeMap, BTreeSet}, marker::PhantomData};
use bevy::ecs::{system::{CommandQueue, Spawn, Insert, Remove, Despawn}, query::With, prelude::Entity};
use eframe::egui;
use crate::{world::{sim::SimulationData, person::{PersonBundle, Person, Personality}, common::{Name, Age, Important}, defs::species::{Species, AssociatedSpecies}, living::Living, time::TimeLength}, gui::{EntityStringHashable, AppMemory}};

use super::widgets::time_length_drag_value;

const SEARCH_KEY: &str = "edit_people_search";

pub(super) fn edit_people_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    ui.horizontal(|ui| {
        if ui.button("Add person").clicked() {
            queue.push(Spawn { bundle: (
                PersonBundle {
                    person: Person,
                    personality: Personality::default(),
                    name: Name("John Doe".to_owned()),
                    age: Age(TimeLength::from_years(32)),
                    state: Living::Alive,
                },
            )});
        };

        if let Some(value) = memory.string_map.get_mut(SEARCH_KEY) {
            egui::TextEdit::singleline(value).hint_text("Enter a search term...").show(ui);
        } else {
            memory.string_map.insert(SEARCH_KEY.to_string(), "".to_string());
        };
    });

    ui.separator();

    character_editor(ui, memory, queue, sim);
}

fn character_editor(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    let mut people_query = sim.app.world.query_filtered::<(Entity, &mut Name, Option<&Important>, &mut Age, &mut Personality, Option<&mut AssociatedSpecies>, &mut Living), With<Person>>();
    let mut people_set: BTreeSet<Entity> = BTreeSet::new();

    for x in people_query.iter(&mut sim.app.world) {
        people_set.insert(x.0);
    }

    let mut species_query = sim.app.world.query::<(Entity, &Name, &Species)>();
    let mut species_map: BTreeMap<Entity, (Name, Species)> = BTreeMap::new();

    for (entity, name, species) in species_query.iter(&sim.app.world) {
        // Non-humanoids are irrelevant here
        if !species.humanoid { continue; }

        // TODO: Figure out a better solution than cloning. Maybe storing refs? Bevy probably has a solution somewhere.
        species_map.insert(entity, (name.clone(), species.clone()));
    }

    let search_term = memory.string_map.get(SEARCH_KEY);

    egui::ScrollArea::both()
    .id_source("people_edit")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for entity in people_set.iter() {
            let (entity, mut name, important, mut age, mut personality, species, mut living) = people_query.get_mut(&mut sim.app.world, *entity).unwrap();

            // Filter options by name
            if search_term.is_some() {
                let search_term = search_term.unwrap().to_lowercase();
                if !search_term.is_empty() && !name.0.to_lowercase().contains(&search_term) {
                    continue;
                }
            }

            // New header for each person
            egui::CollapsingHeader::new(format!("{} ({:?})", name.0, entity))
            .id_source(EntityStringHashable(entity, "person_cfg".to_string()))
            .show(ui, |ui| {
                // Danger zone buttons
                ui.horizontal(|ui| {
                    if ui.button("Delete person").clicked() {
                        queue.push(Despawn { entity });
                    }
                });

                ui.add_space(3.0);

                // General details
                egui::Grid::new(EntityStringHashable(entity, "general_details".to_string()))
                .spacing([16.0, 6.0])
                .striped(true)
                .show(ui, |ui| {
                    // Person's name
                    ui.label("Name");
                    ui.text_edit_singleline(&mut name.0);
                    ui.end_row();

                    // Person's state (like being alive)
                    ui.label("State");
                    ui.horizontal(|ui| {
                        if ui.button(format!("{:?}", *living)).clicked() {
                            *living = match *living {
                                Living::Alive => Living::Dead,
                                Living::Dead => Living::Alive,
                            }
                        }
                    });
                    ui.end_row();

                    // Is important
                    ui.label("Importance");
                    let mut is_important: bool = important.is_some();
                    ui.add(egui::Checkbox::new(&mut is_important, "Generate extra information"));
                    if is_important && important.is_none() {
                        queue.push(Insert { entity, bundle: Important });
                    } else if !is_important && important.is_some() {
                        queue.push(Remove::<Important> { entity, phantom: PhantomData });
                    }
                    ui.end_row();

                    // Personality
                    ui.label("Personality");
                    ui.vertical(|ui| {
                        // Split borrow, because borrow checker
                        let split = personality.split_borrow();

                        // Sliders for personality values
                        egui::Grid::new(EntityStringHashable(entity, "personality_items".to_string()))
                        .show(ui, |ui| {
                            // prevent code repetition with iteration
                            for (value, left_text, right_text) in [
                                (split.0, "Selfishness", "Selflessness"),
                                (split.1, "Timidity", "Aggression"),
                            ] {
                                ui.label(left_text);
                                ui.add(egui::Slider::new(value, 0.0..=1.0).show_value(false));
                                ui.label(right_text);
                                ui.end_row();
                            }
                        });
                    });
                    ui.end_row();

                    // Species related values
                    let mut use_slider: bool = false;
                    let mut age_of_maturity: TimeLength = TimeLength::from_years(0);
                    let mut max_age: TimeLength = TimeLength::from_years(1);

                    // Species
                    ui.label("Species");
                    ui.horizontal(|ui| {
                        if species.is_some() {
                            let mut associated_species = species.unwrap();
                            if !species_map.contains_key(&associated_species.0) {
                                queue.push(Remove::<AssociatedSpecies> { entity, phantom: PhantomData });
                            } else {
                                // Get values to clamp min and max age
                                let (_, species_info) = species_map.get(&associated_species.0).unwrap();
                                use_slider = species_info.humanoid;
                                age_of_maturity = species_info.maturity_age;
                                max_age = species_info.max_age;

                                // Species selection
                                egui::ComboBox::from_id_source(EntityStringHashable(entity, "species_selection_box".to_string()))
                                .selected_text(&species_map.get(&associated_species.0).unwrap().0.0)
                                .show_ui(ui, |ui| {
                                    for (entity, (name, _species)) in species_map.iter() {
                                        ui.selectable_value(&mut associated_species.0, *entity, name.0.clone());
                                    }
                                });

                                // Remove button
                                if ui.button("Remove species").clicked() {
                                    queue.push(Remove::<AssociatedSpecies> { entity, phantom: PhantomData });
                                }
                            }
                        } else {
                            if species_map.is_empty() {
                                ui.label("No humanoid species defined");
                            } else {
                                if ui.button("Add species").clicked() {
                                    // Add species
                                    queue.push(Insert { entity, bundle: AssociatedSpecies(species_map.iter().nth(0).unwrap().0.clone()) });
                                }
                            }
                        }
                    });
                    ui.end_row();

                    // Adjust age as per species
                    age.0 = age.0.min(max_age);

                    // Age
                    ui.label("Age");
                    if use_slider {
                        ui.vertical(|ui| {
                            ui.add(egui::Slider::new(&mut age.0, TimeLength::ZERO..=max_age));
                            if age.0 >= age_of_maturity {
                                ui.label("This person is an adult for their species.");
                            } else {
                                ui.label("This person is a child for their species.");
                            }
                        });
                    } else {
                        ui.add(time_length_drag_value(&mut age.0).clamp_range(TimeLength::ZERO..=max_age));
                    }
                    ui.end_row();
                });
            });
        }
    });
}