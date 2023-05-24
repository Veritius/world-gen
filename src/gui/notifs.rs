//! "Notification" elements

use eframe::{egui::{self, Id, Frame, Margin, LayerId}, epaint::{Color32, FontId, Rect, Pos2}};

pub enum NotificationType {
    Info,
    Warning,
    Error,
}

pub struct Notification {
    remaining_time: f32,
    original_time: f32,

    text: String,
    kind: NotificationType,
}

impl Notification {
    pub fn new(text: impl Into<String>, time: f32, kind: NotificationType) -> Notification {
        Self {
            remaining_time: time,
            original_time: time,

            text: text.into(),
            kind,
        }
    }
}

/// Decrements notification timers and removes them eventually.
/// Only call once.
pub fn update_notifications(
    notifs: &mut Vec<Notification>,
    ctx: &egui::Context,
) {
    // Decrement timers
    let time = ctx.input(|i: &egui::InputState| i.stable_dt);
    let mut removalvec = vec![];
    for (idx, notif) in notifs.iter_mut().enumerate() {
        notif.remaining_time -= time;
        if notif.remaining_time < 0.0 {
            removalvec.push(idx);
        }
    }

    // Reverse iteration to remove later items before earlier ones to avoid changing indexes
    for id in removalvec.iter().rev() {
        notifs.remove(*id);
    }
}

/// Lists notifications
pub fn show_notifications(
    notifs: &Vec<Notification>,
    ctx: &egui::Context, 
) {
    // Don't bother if there's no notifications, and always repaint if there are (so they disappear)
    if notifs.len() == 0 { return; } else { ctx.request_repaint(); }

    // Show notifications
    egui::TopBottomPanel::bottom("warnings").show(ctx, |ui| {
        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.add_space(2.0);
            ui.horizontal(|ui| {
                // Iterate all notifications and display them
                for notif in notifs.iter() {
                    let prefix = match notif.kind {
                        NotificationType::Info => "info",
                        NotificationType::Warning => "warning",
                        NotificationType::Error => "error",
                    };

                    let fill_color = match notif.kind {
                        NotificationType::Info => Color32::from_rgb(53, 150, 234),
                        NotificationType::Warning => Color32::from_rgb(234, 225, 44),
                        NotificationType::Error => Color32::from_rgb(237, 61, 61),
                    };

                    let text_color = match notif.kind {
                        NotificationType::Info => Color32::WHITE,
                        NotificationType::Warning => Color32::BLACK,
                        NotificationType::Error => Color32::WHITE,
                    };

                    egui::Frame::none()
                    .fill(fill_color)
                    .rounding(2.0)
                    .inner_margin(Margin::symmetric(2.0, 1.0))
                    .show(ui, |ui| {
                        ui.label(egui::RichText::new(format!("({prefix}) {}", notif.text)).color(text_color));
                    });
                }
            });
        });
    });
}