use bevy::ecs::{system::{CommandQueue, Spawn, Despawn}, prelude::Entity, world::Mut};
use eframe::egui;
use crate::{world::{sim::SimulationData, defs::species::{SpeciesBundle, Species}, common::Name, time::Age}, gui::{EntityStringHashable, edit::widgets::{time_length_drag_value, time_length_slider}}};

const SOFT_MAX_AGE: Age = Age::from_years(10_000);
// This is the age at which a humanoid is considered an 'adult', and therefore can reproduce.
// DO NOT set this lower. No excuses.
const MIN_HUMANOID_AGE: Age = Age::from_years(18);

pub(super) fn species_menu(
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
                        maturity_age: Age::from_years(3),
                        max_age: Age::from_years(12),
                        resilience: 15.0,
                        immunity: 1.0,
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
                        max_age: Age::from_years(100),
                        resilience: 100.0,
                        immunity: 1.0,
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
            let range = if species.humanoid { MIN_HUMANOID_AGE..=SOFT_MAX_AGE } else { Age::ZERO..=SOFT_MAX_AGE };
            ui.add(time_length_drag_value(&mut species.max_age).clamp_range(range));
            ui.end_row();

            let max_age = species.max_age.clone();

            // Age of maturity
            ui.label("Age of maturity");
            let range = if species.humanoid { MIN_HUMANOID_AGE..=max_age } else { Age::ZERO..=max_age };
            species.maturity_age = species.maturity_age.max(*range.start());
            ui.add(time_length_slider(&mut species.maturity_age, range));
            ui.end_row();

            // How much this species can endure before death
            ui.label("Resilience");
            ui.add(egui::Slider::new(&mut species.resilience, 1.0..=100000.0).logarithmic(true).step_by(1.0));
            ui.end_row();

            // How slowly afflictions progress for this species
            ui.label("Immunity");
            let text = match species.immunity {
                x if x <= 0.1 => "Super-immune",
                x if x < 0.8 => "Enhanced immunity",
                x if x < 1.2 => "About average",
                x if x < 2.0 => "Vulnerable",
                x if x >= 2.0 => "Extremely vulnerable",
                _ => "",
            };
            ui.add(egui::Slider::new(&mut species.immunity, 0.001..=3.0).text(text));
            ui.end_row();
        });
    });
}