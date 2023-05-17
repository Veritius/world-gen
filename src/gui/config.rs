use std::{collections::BTreeMap, marker::PhantomData};

use bevy_ecs::prelude::*;
use eframe::egui;
use rand::Rng;
use crate::world::{soft_limits::{MAX_YEARS_TO_SIMULATE, MIN_YEARS_TO_SIMULATE}, WorldPregenConfig, person::{Person, PersonBundle}, thing::{Age, Name, Important}, defs::{Species, SpeciesBundle, AssociatedSpecies}};

use super::{KnockoffCommandQueue, DeleteEntity, InsertComponent, EntityStringHashable, RemoveComponent};

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

    let mut cmd = KnockoffCommandQueue(Vec::new());

    match &state.tab {
        Tab::Meta => tab_meta(ui, state, &mut cmd),
        Tab::People { search_filter: _ } => tab_people(ui, state, &mut cmd),
        Tab::Events => todo!(),
        Tab::Places => todo!(),
        Tab::Definitions => tab_defs(ui, state, &mut cmd),
    }

    cmd.execute(&mut state.world);
}

fn tab_meta(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
    _cmd: &mut KnockoffCommandQueue,
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
    cmd: &mut KnockoffCommandQueue,
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
                cmd.push(Box::new(DeleteEntity(entity)));
            }
        }

        egui::TextEdit::singleline(search_filter).hint_text("Search by name").show(ui);
    });

    ui.separator();

    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
        // List of available species
        let mut available_species: BTreeMap<Entity, String> = BTreeMap::new();
        
        // Add species to map
        let mut query = state.world.query::<(Entity, &Name, &Species)>();
        for (entity, name, species) in query.iter(&mut state.world) {
            if species.humanoid {
                available_species.insert(entity, name.0.clone());
            }
        }

        let mut query = state.world.query_filtered::<(Entity, &mut Name, Option<&mut Age>, Option<&mut AssociatedSpecies>), With<Person>>();
        for (entity, mut name, age, species) in query.iter_mut(&mut state.world) {
            // Filter out entries
            if search_filter != "" && !name.0.to_lowercase().starts_with(&*search_filter.to_lowercase()) { return; }

            egui::CollapsingHeader::new(name.0.clone()).id_source(entity).show(ui, |ui| {
                // Name
                ui.horizontal(|ui| {
                    ui.label("Name");
                    ui.text_edit_singleline(&mut name.0);
                });

                // Age
                ui.horizontal(|ui| {
                    ui.label("Age");
                    if age.is_some() {
                        let mut age = age.unwrap();
                        ui.add(egui::DragValue::new(&mut age.0).suffix(" years old"));
                        if ui.button("Make ageless").clicked() {
                            cmd.push(Box::new(RemoveComponent::<Age>(entity, PhantomData)));
                        }
                    } else {
                        if ui.button("Add age").clicked() {
                            cmd.push(Box::new(InsertComponent { entity, component: Age(u32::MIN) }));
                        }
                    }
                });


                // Species
                ui.horizontal(|ui| {
                    ui.label("Species");
                    if let Some(mut species) = species {
                        let spec = available_species.get(&species.0.clone());
                        if spec.is_some() {
                            egui::ComboBox::new(EntityStringHashable(entity, "species_choice_box".to_string()), "")
                            .selected_text(spec.unwrap())
                            .show_ui(ui, |ui| {
                                // List all humanoid species as options
                                for (species_entity, species_name) in &available_species {
                                    ui.selectable_value(&mut species.as_mut().0, *species_entity, species_name);
                                }
                            });
                            if ui.button("Remove").clicked() {
                                cmd.push(Box::new(RemoveComponent::<AssociatedSpecies>(entity, PhantomData)));
                            }
                        } else {
                            cmd.push(Box::new(RemoveComponent::<AssociatedSpecies>(entity, PhantomData)));
                        }
                    } else {
                        ui.add_enabled_ui(available_species.len() > 0, |ui| {
                            if ui.button("Add species").clicked() {
                                cmd.push(Box::new(InsertComponent { entity: entity.clone(), component: AssociatedSpecies(available_species.iter().nth(0).unwrap().0.clone()) }));
                            };
                        });
                    } 
                });
                    
                if ui.button("Delete").clicked() {
                    cmd.push(Box::new(DeleteEntity(entity)));
                }
            });
        }
    });

    } // enum break
}

fn tab_defs(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
    cmd: &mut KnockoffCommandQueue,
) {
    egui::ScrollArea::vertical().auto_shrink([false, false]).show(ui, |ui| {
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
                        cmd.push(Box::new(DeleteEntity(entity)));
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
                        cmd.push(Box::new(DeleteEntity(entity)));
                    }
                });
            }
        });
    });
}