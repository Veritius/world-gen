use bevy_ecs::prelude::*;
use eframe::egui;
use rand::Rng;
use crate::world::{soft_limits::{MAX_YEARS_TO_SIMULATE, MIN_YEARS_TO_SIMULATE}, WorldPregenConfig, person::{Person, Name}, thing::Age};

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
    People,
    Events,
    Places,
}

pub fn config_ui(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab, Tab::Meta, "Meta");
        
        ui.selectable_value(&mut state.tab, Tab::People, "People");
    });

    ui.separator();

    match &state.tab {
        Tab::Meta => tab_meta(ui, state),
        Tab::People => tab_people(ui, state),
        Tab::Events => todo!(),
        Tab::Places => todo!(),
    }
}

fn tab_meta(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
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
) {
    ui.horizontal(|ui| {
        if ui.button("Add person").clicked() {
            state.world.spawn((
                Person,
                Name("Some Body".to_string()),
                Age(32),
            ));
        }

        if ui.button("Clear people").clicked() {
            let mut query = state.world.query_filtered::<Entity, With<Person>>();

            let mut entities = vec![];
            for entity in query.iter(&state.world) {
                entities.push(entity);
            }

            for entity in entities {
                state.world.despawn(entity);
            }
        }
    });

    ui.separator();

    ui.horizontal_wrapped(|ui| {
        let mut query = state.world.query_filtered::<(&mut Name, Option<&mut Age>), With<Person>>();
        for (mut name, age) in query.iter_mut(&mut state.world) {
            ui.group(|ui| { ui.vertical(|ui| {
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
            })});
        }
    });
}