use std::collections::BTreeMap;
use bevy::ecs::{system::CommandQueue, world::Mut};
use eframe::egui;
use crate::world::{sim::SimulationData, defs::{SimulationConfig, HistoryDirection, Timespan}};

pub(super) fn edit_meta_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    _queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    let mut config = sim.app.world.resource_mut::<SimulationConfig>();

    ui.add_enabled_ui(!config.locked_in, |ui| world_settings(ui, &mut config));

    ui.separator();

    ui.horizontal(|ui| {
        ui.add_enabled_ui(config.increments_completed < config.increments_for_completion, |ui| {
            if ui.button("Begin simulation").clicked() {
                state.insert("try_execute_simulation".to_string(), "true".to_string());
            }
        });

        ui.add_enabled_ui(config.increments_completed != 0, |ui| {
            if ui.button("Reset incrementor").clicked() {
                config.increments_completed = 0;
            }
        });

        ui.label(format!("{} out of {} steps complete", config.increments_completed, config.increments_for_completion));
    });
}

fn world_settings(
    ui: &mut egui::Ui,
    config: &mut Mut<SimulationConfig>,
) {
    egui::ScrollArea::both()
    .id_source("world_settings_scroll")
    .auto_shrink([false, true])
    .show(ui, |ui| {
        egui::Grid::new("world_settings_grid")
        .spacing([10.0, 3.0])
        .striped(true)
        .show(ui, |ui| {
            // World name
            ui.label("World name");
            ui.text_edit_singleline(&mut config.name).on_hover_text(
                "The name of the world. This is purely cosmetic, and doesn't affect the simulation."
            );
            ui.end_row();

            // World seed
            ui.label("Random seed");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut config.seed));
                if ui.button("Regenerate").clicked() {
                    config.seed = rand::random();
                }
            }).response.on_hover_text(
                "The random seed of the world. Using the same seed, along with other input parameters, will produce an identical world."
            );
            ui.end_row();

            // History generation direction
            ui.label("Direction");
            egui::ComboBox::from_id_source("world_settings_direction")
            .selected_text(format!("{:?}", config.direction))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut config.direction, HistoryDirection::Forwards, "Forwards");
                ui.selectable_value(&mut config.direction, HistoryDirection::Backwards, "Backwards");
            }).response.on_hover_text(
                "The direction in which history generates.
Generating forwards will simulate as if time is progressing from a single instant you define, creating an evolving and unpredictable world.
Generating backwards will generate the history leading to the present, explaining the current state of the world. This is much more restrained than generating forwards."
            );
            ui.end_row();

            // History step length
            ui.label("Timespan");
            egui::ComboBox::from_id_source("world_settings_timespan")
            .selected_text(format!("{:?}", config.timespan))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut config.timespan, Timespan::Months, "Months");
                ui.selectable_value(&mut config.timespan, Timespan::Days, "Days");
            }).response.on_hover_text(
                "The span of time each tick of the simulator works at.
Ticking by months will finish simulating faster, but provides less detail, and is best chosen for large periods of time.
Ticking by days simulates slowly, but provides a lot of detail, and is best chosen for smaller periods of time."
            );
            ui.end_row();

            // Simulation steps
            ui.label(format!("{:?} to simulate", config.timespan));
            ui.horizontal(|ui| {
                ui.add(egui::Slider::new(&mut config.increments_for_completion, 10..=10_000).logarithmic(true));
                const STEPS_BEFORE_WARNING: u32 = 1000;
                if config.increments_for_completion > STEPS_BEFORE_WARNING {
                    ui.label(egui::RichText::from("⚠").color(egui::Color32::RED)).on_hover_text(
                        format!("Setting the amount of steps in the simulation above {} may take a long time.", STEPS_BEFORE_WARNING)
                    );
                }
            });
            ui.end_row();
        });
    });
}