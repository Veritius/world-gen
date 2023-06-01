use bevy::{ecs::system::{CommandQueue, SystemState, Spawn, Despawn}, prelude::{Query, Entity, Mut}};
use eframe::egui;
use crate::{world::{sim::SimulationData, living::afflictions::{Affliction, AfflictionBundle, HealthAdjustmentFunction}, common::Name}, gui::EntityStringHashable};

pub(super) fn afflictions_menu(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    sim: &mut SimulationData,
) {
    // New humanoid species
    if ui.button("New affliction").clicked() {
        queue.push(Spawn {
            bundle: AfflictionBundle {
                name: Name("A new affliction".to_string()),
                affliction: Affliction::default(),
            }
        });
    }

    ui.separator();

    let mut state: SystemState<Query<(Entity, &mut Name, &mut Affliction)>> = SystemState::new(&mut sim.app.world);
    let mut state_mut = state.get_mut(&mut sim.app.world);
    
    egui::ScrollArea::both()
    .id_source("species_scroll_area")
    .auto_shrink([false, false])
    .show(ui, |ui| {
        for query_data in state_mut.iter_mut() {
            affliction_editor(ui, queue, query_data);
        }
    });
}

fn affliction_editor(
    ui: &mut egui::Ui,
    queue: &mut CommandQueue,
    query_data: (Entity, Mut<Name>, Mut<Affliction>),
) {
    let (entity, mut name, mut affliction) = query_data;

    egui::CollapsingHeader::new(format!("{} ({:?})", &name.0, entity))
    .id_source(EntityStringHashable(entity, "affliction_editor_section".to_owned()))
    .show(ui, |ui| {
        ui.horizontal(|ui| {
            if ui.button("Delete species").clicked() {
                queue.push(Despawn { entity });
            }
        });

        ui.add_space(3.0);

        // Species details
        egui::Grid::new(EntityStringHashable(entity, "affliction_editor_details".to_owned()))
        .min_col_width(20.0)
        .spacing([15.0, 3.0])
        .striped(true)
        .show(ui, |ui| {
            ui.label("Name");
            ui.text_edit_singleline(&mut name.0);
            ui.end_row();

            ui.label("Flat change");
            health_adjustment_function_selector(ui, "affliction_editor_flat_rate", entity, &mut affliction.flat, 0.0);
            ui.end_row();

            ui.label("Coefficient");
            health_adjustment_function_selector(ui, "affliction_editor_coefficient", entity, &mut affliction.coefficient, 1.0);
            ui.end_row();
        });
    });
}

fn health_adjustment_function_selector(
    ui: &mut egui::Ui,
    ukey: impl Into<String>,
    entity: Entity,
    value: &mut HealthAdjustmentFunction,
    static_default: f32,
) {
    let selected_text = match value {
        HealthAdjustmentFunction::NoAdjustment => "No adjustment",
        HealthAdjustmentFunction::Scaling(_) => "Scaling value",
        HealthAdjustmentFunction::Static(_) => "Static value",
        HealthAdjustmentFunction::Custom(_) => "Function",
    };

    ui.horizontal(|ui| {
        egui::ComboBox::new(EntityStringHashable::new(entity, ukey), "")
        .selected_text(selected_text)
        .show_ui(ui, |ui| {
            for (text, new) in [
                ("No adjustment", HealthAdjustmentFunction::NoAdjustment),
                ("Scaling value", HealthAdjustmentFunction::Scaling(static_default)),
                ("Static value", HealthAdjustmentFunction::Scaling(static_default)),
                // Custom is intentionally not added here, as it can't be edited in the UI
            ] {
                if ui.button(text).clicked() {
                    *value = new;
                }
            }
        });

        match value {
            HealthAdjustmentFunction::NoAdjustment => ui.label(egui::RichText::new("Nothing to adjust.").italics()),
            HealthAdjustmentFunction::Scaling(value) => ui.add(egui::DragValue::new(value).speed(0.1)),
            HealthAdjustmentFunction::Static(value) => ui.add(egui::DragValue::new(value).speed(0.1)),
            HealthAdjustmentFunction::Custom(_) => ui.label(egui::RichText::new("Can't edit functions.").italics()),
        }
    });
}