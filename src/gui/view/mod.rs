use std::{sync::RwLockReadGuard, collections::BTreeMap};
use eframe::egui::{self, plot::{PlotPoints, Line, Plot, PlotUi, Legend, Corner}};
use crate::world::sim::{SimulationBoundary, RECORD_LENGTH};
use super::AppMemory;

pub(super) fn view_ui(
    ui: &mut egui::Ui,
    memory: &mut AppMemory,
    sim: RwLockReadGuard<SimulationBoundary>,
) {
    ui.horizontal(|ui| {
        if ui.button("Stop simulation").clicked() {
            memory.markers.insert("try_freeze_simulation".to_owned());
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
    ui.label("Time taken per simulation step (in seconds)");
    Plot::new("tick_time_plot")
    .height(100.0)
    .allow_drag(false)
    .allow_scroll(false)
    .allow_boxed_zoom(false)
    .include_x(0.0)
    .include_x(RECORD_LENGTH as f64)
    .include_y(0.0)
    .include_y(0.005)
    .show_x(false)
    .label_formatter(|name, value| {
        if value.y < 1.0 {
            format!("{}\n{:.3} milliseconds", name, (value.y * 1000.0))
        } else {
            format!("{}\n{:.4} seconds", name, value.y)
        }
    })
    .x_axis_formatter(|_, _| format!(""))
    .show(ui, |plot_ui| {
        let points: PlotPoints = sim.tick_time_history.iter().enumerate().map(|(i, val)| { [i as f64, *val] }).collect();
        let line = Line::new(points);
        plot_ui.line(line);
    });

    ui.add_space(10.0);

    // Entity count
    ui.label("Entity count");
    
    // Stats that will be shown in the graph
    let stats = [
        ("Total", &sim.entity_count_history),
        ("People", &sim.people_count_history),
        ("Places", &sim.place_count_history),
    ];

    // Plot
    Plot::new("entity_count_plot")
    .height(150.0)
    .allow_drag(false)
    .allow_scroll(false)
    .allow_boxed_zoom(false)
    .include_x(0.0)
    .include_x(RECORD_LENGTH as f64)
    .show_x(false)
    .legend(Legend::default().position(Corner::LeftTop))
    .label_formatter(|name, value| format!("{}\n{}", name, value.y.floor()))
    .y_axis_formatter(|_, _| format!(""))
    .x_axis_formatter(|_, _| format!(""))
    .show(ui, |plot_ui| {
        for (name, stat) in stats {
            let points: PlotPoints = stat.iter().enumerate().map(|(i, val)| [i as f64, *val]).collect();
            let line = Line::new(points).name(name);
            plot_ui.line(line);
        }
    });
}