use std::{sync::RwLockReadGuard, collections::BTreeMap};
use eframe::egui::{self, plot::{PlotPoints, Line, Plot, PlotUi}};
use crate::world::sim::SimulationBoundary;

pub(super) fn view_ui(
    ui: &mut egui::Ui,
    state: &mut BTreeMap<String, String>,
    sim: RwLockReadGuard<SimulationBoundary>,
) {
    ui.horizontal(|ui| {
        if ui.button("Stop simulation").clicked() {
            state.insert("try_freeze_simulation".to_string(), "".to_string());
        }

        let percent = sim.steps_complete as f32 / sim.steps_total as f32;
        ui.add(egui::ProgressBar::new(percent).show_percentage());
    });

    ui.separator();

    stat_plots(ui, sim);
}

fn stat_plots(
    ui: &mut egui::Ui,
    sim: RwLockReadGuard<SimulationBoundary>,
) {
    // Tick time display
    ui.label("Time taken per simulation step");
    Plot::new("tick_time_plot")
    .height(100.0)
    .show(ui, |plot_ui| {
        let points: PlotPoints = sim.tick_time_history.iter().enumerate().map(|(i, val)| { [i as f64, *val] }).collect();
        let line = Line::new(points);
        plot_ui.line(line);
    });

    ui.add_space(10.0);

    // Entity count
    ui.label("Entity count");
    ui.horizontal(|ui| {
        // Stats that will be shown in the graph
        let stats = [
            ("Entities", &sim.entity_count_history),
            ("People", &sim.people_count_history),
        ];

        // Plot
        Plot::new("entity_count_plot")
        .width(ui.available_width() - 160.0)
        .allow_drag(false)
        .allow_scroll(false)
        .allow_boxed_zoom(false)
        .show_x(false)
        .show_y(false)
        .show(ui, |plot_ui| {
            for (_, stat) in stats {
                display_u32s_on_plot(plot_ui, stat);
            }
        });
        
        // Number stats
        ui.vertical(|ui| {
            for (name, stat) in stats {
                ui.label(format!("{}: {}", name, stat.iter().last().unwrap_or(&0u32)));
            }
        });
    });
}

/// Takes a vec of u32s and puts them on the plot widget
fn display_u32s_on_plot(
    ui: &mut PlotUi,
    set: &Vec<u32>,
) {
    let points: PlotPoints = set.iter().enumerate().map(|(i, val)| [i as f64, *val as f64]).collect();
    let line = Line::new(points);
    ui.line(line);
}