use eframe::epaint::Color32;
use replace_with::replace_with_or_abort;
use crate::world::sim::Simulation;
use super::{WorldGenApp, systems_check, modal::ModalWindow, notifs::{Notification, NotificationType}};

/// Deals with changing the simulation from the UI
pub(super) fn simulation_fns(
    app: &mut WorldGenApp
) {
    // Start simulation
    if app.memory.markers.contains("try_execute_simulation") {
        app.memory.markers.remove("try_execute_simulation");

        match app.simulation.current_or_err() {
            Ok(simulation) => {
                systems_check(simulation);
            },
            Err(_error) => todo!(),
        }

        replace_with_or_abort(&mut app.simulation, |sim| sim.try_execute().0);
    }

    // Freeze simulation
    if app.memory.markers.contains("try_freeze_simulation") {
        app.memory.markers.remove("try_freeze_simulation");
        replace_with_or_abort(&mut app.simulation, |sim| {
            match sim.freeze() {
                Ok(success) => {
                    app.memory.modal_popup = Some(ModalWindow::new("The simulation successfully exited.").outline_color(Color32::LIGHT_GREEN));
                    success
                },
                Err(error) => {
                    let errorstr = format!("Simulation encountered an error: {:?}", error);
                    app.memory.modal_popup = Some(ModalWindow::new(&errorstr).outline_color(Color32::LIGHT_RED));
                    app.memory.notifications.push(Notification::new(&errorstr, 20.0, NotificationType::Error));
                    Simulation::default() // create new default sim
                },
            }
        });
    }
}