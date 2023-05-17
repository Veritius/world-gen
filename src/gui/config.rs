use std::collections::HashSet;

use bevy_ecs::prelude::*;
use eframe::egui;
use rand::Rng;
use crate::world::{soft_limits::{MAX_YEARS_TO_SIMULATE, MIN_YEARS_TO_SIMULATE}, WorldPregenConfig, person::{Person, PersonBundle}, thing::{Age, Name, Important}, defs::{Species, SpeciesBundle}};

pub struct ConfigState {
    tab: Tab,
    world: World,
}

impl Default for ConfigState {
    fn default() -> Self {
        let mut world = World::new();
        world.init_resource::<WorldPregenConfig>();

        Self {
            tab: Default::default(),
            world,
        }
    }
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tab {
    #[default]
    Meta,
    People {
        search_filter: String,
    },
    Events,
    Places,
    Definitions,
}

pub fn config_ui(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab, Tab::Meta, "Meta");
        ui.selectable_value(&mut state.tab, Tab::People { search_filter: "".to_string() }, "People");
        ui.selectable_value(&mut state.tab, Tab::Definitions, "Definitions");
    });

    ui.separator();

    let mut to_delete = HashSet::new();

    match &state.tab {
        Tab::Meta => tab_meta(ui, state, &mut to_delete),
        Tab::People { search_filter: _ } => tab_people(ui, state, &mut to_delete),
        Tab::Events => todo!(),
        Tab::Places => todo!(),
        Tab::Definitions => tab_defs(ui, state, &mut to_delete),
    }

    for entity in to_delete {
        state.world.despawn(entity);
    }
}

fn tab_meta(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
    to_delete: &mut HashSet<Entity>,
) {
    let world = &mut *state.world.resource_mut::<WorldPregenConfig>();

    ui.horizontal(|ui| {
        ui.label("World name");
        ui.add(egui::TextEdit::singleline(&mut world.name));
    });

    ui.horizontal(|ui| {
        ui.label("Random seed");
        ui.add(egui::DragValue::new(&mut world.random_seed).speed(2.5));
        if ui.button("New seed").clicked() {
            world.random_seed = rand::thread_rng().gen::<u32>();
        }
    });

    ui.horizontal(|ui| {
        ui.label("History start year");
        ui.add(egui::Slider::new(&mut world.history_starts_at, 0..=3000));
    });

    ui.horizontal(|ui| {
        ui.label("Years to simulate");
        ui.add(egui::Slider::new(&mut world.years_to_simulate, MIN_YEARS_TO_SIMULATE..=MAX_YEARS_TO_SIMULATE));
    });

    ui.horizontal(|ui| {
        ui.label("History generation direction");
        egui::ComboBox::new("history_generation_direction", "")
        .selected_text(format!("{:?}", &mut world.generation_direction))
        .show_ui(ui, |ui| {
            ui.selectable_value(&mut world.generation_direction, crate::world::GenerationDirection::Forwards, "Forwards");
            ui.selectable_value(&mut world.generation_direction, crate::world::GenerationDirection::Backwards, "Backwards");
        });
    });

    ui.horizontal(|ui| {
        ui.label("Chaos multiplier");
        ui.add(egui::Slider::new(&mut world.chaos_multiplier, 0.1..=2.0));
    });
}

fn tab_people(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
    to_delete: &mut HashSet<Entity>,
) {
    if let Tab::People { ref mut search_filter } = &mut state.tab { // enum start

    ui.horizontal(|ui| {
        if ui.button("Add person").clicked() {
            state.world.spawn((
                PersonBundle {
                    person: Person,
                    name: Name("Urist McHands".to_string()),
                    age: Age(44),
                },
                // Custom characters will always be marked important.
                Important,
            ));
        }

        if ui.button("Clear people").clicked() {
            let mut query = state.world.query_filtered::<Entity, With<Person>>();

            for entity in query.iter(&state.world) {
                to_delete.insert(entity);
            }
        }

        egui::TextEdit::singleline(search_filter).hint_text("Search by name").show(ui);
    });

    ui.separator();

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        let mut query = state.world.query_filtered::<(Entity, &mut Name, Option<&mut Age>), With<Person>>();
        for (entity, mut name, age) in query.iter_mut(&mut state.world) {
            // Filter out entries
            if search_filter != "" && !name.0.to_lowercase().starts_with(&*search_filter.to_lowercase()) { return; }

            egui::CollapsingHeader::new(name.0.clone()).id_source(entity).show(ui, |ui| {
                // Name
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut name.0);
                });

                // Age
                if age.is_some() {
                    ui.horizontal(|ui| {
                        ui.label("Age");
                        let mut age = age.unwrap();
                        ui.add(egui::DragValue::new(&mut age.0));
                    });
                }
                    
                if ui.button("Delete").clicked() {
                    to_delete.insert(entity);
                }
            });
        }
    });

    } // enum break
}

fn tab_defs(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
    to_delete: &mut HashSet<Entity>,
) {
    egui::ScrollArea::vertical().show(ui, |ui| {
        ui.collapsing("Species", |ui| {
            ui.horizontal(|ui| {
                if ui.button("New creature").clicked() {
                    state.world.spawn((
                        SpeciesBundle {
                            name: Name("New creature".to_string()),
                            species: Species {
                                humanoid: false,
                                maturity_age: 2,
                                max_age: 20,
                            },
                        },
                        Important,
                    ));
                }
        
                if ui.button("New humanoid").clicked() {
                    state.world.spawn((
                        SpeciesBundle {
                            name: Name("New humanoid".to_string()),
                            species: Species {
                                humanoid: true,
                                maturity_age: 18,
                                max_age: 100,
                            },
                        },
                        Important,
                    ));
                }

                if ui.button("Delete all").clicked() {
                    let mut query = state.world.query_filtered::<Entity, With<Species>>();
                    for entity in query.iter(&state.world) {
                        to_delete.insert(entity);
                    }
                }
            });
            
            let mut query = state.world.query::<(Entity, &mut Name, &mut Species)>();
            for (entity, mut name, mut species) in query.iter_mut(&mut state.world) {
                egui::CollapsingHeader::new(name.0.clone()).id_source(entity).show(ui, |ui| {
                    // Species name
                    ui.horizontal(|ui| {
                        ui.label("Name");
                        ui.text_edit_singleline(&mut name.0);
                    });
    
                    // Humanoid
                    ui.horizontal(|ui| {
                        ui.label("Humanoid");
                        ui.checkbox(&mut species.humanoid, "");
                    });
    
                    let is_humanoid: bool = species.humanoid;
    
                    // Age of maturity
                    ui.horizontal(|ui| {
                        ui.label("Matures at");
    
                        let mut dragvalue = egui::DragValue::new(&mut species.maturity_age).suffix(" years");
                        if is_humanoid {
                            dragvalue = dragvalue.clamp_range(18..=u32::MAX);
                        }
                        
                        ui.add(dragvalue);
                    });
    
                    let maturity_age: u32 = species.maturity_age;
    
                    // Max age
                    ui.horizontal(|ui| {
                        ui.label("Max age");
                        ui.add(egui::DragValue::new(&mut species.max_age).suffix(" years").clamp_range(maturity_age..=u32::MAX));
                    });

                    // Delete button
                    if ui.button("Delete").clicked() {
                        to_delete.insert(entity);
                    }
                });
            }
        });
    });
}