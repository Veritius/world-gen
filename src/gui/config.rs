use eframe::egui;
use rand::Rng;
use crate::world::{WorldPregenConfig, soft_limits::{MAX_YEARS_TO_SIMULATE, MIN_YEARS_TO_SIMULATE}};

#[derive(Default)]
pub struct ConfigState {
    tab: Tab,
    world: WorldPregenConfig,
}

#[derive(Default, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tab {
    #[default]
    Meta,
}

pub fn config_ui(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.horizontal(|ui| {
        ui.selectable_value(&mut state.tab, Tab::Meta, "Meta");
    });

    ui.separator();

    match &state.tab {
        Tab::Meta => tab_meta(ui, state),
    }
}

fn tab_meta(
    ui: &mut egui::Ui,
    state: &mut ConfigState,
) {
    ui.horizontal(|ui| {
        ui.label("World name");
        ui.add(egui::TextEdit::singleline(&mut state.world.name));
    });

    ui.horizontal(|ui| {
        ui.label("Random seed");
        ui.add(egui::DragValue::new(&mut state.world.random_seed).speed(2.5));
        if ui.button("New seed").clicked() {
            state.world.random_seed = rand::thread_rng().gen::<u32>();
        }
    });

    ui.horizontal(|ui| {
        ui.label("History start year");
        ui.add(egui::Slider::new(&mut state.world.history_starts_at, 0..=3000));
    });

    ui.horizontal(|ui| {
        ui.label("Years to simulate");
        ui.add(egui::Slider::new(&mut state.world.years_to_simulate, MIN_YEARS_TO_SIMULATE..=MAX_YEARS_TO_SIMULATE));
    });

    ui.horizontal(|ui| {
        ui.label("Chaos multiplier");
        ui.add(egui::Slider::new(&mut state.world.chaos_multiplier, 0.1..=2.0));
    });
}