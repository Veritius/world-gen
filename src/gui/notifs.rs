//! "Notification" elements

use eframe::{egui::{self, Id, Frame, Margin, LayerId}, epaint::{Color32, FontId, Rect, Pos2}};

pub enum NotificationType {
    Info,
    Warning,
    Error,
}

pub struct Notification {
    uid: Id,

    remaining_time: f32,
    original_time: f32,

    text: String,
    kind: NotificationType,
}

impl Notification {
    pub fn new(text: impl Into<String>, time: f32, kind: NotificationType) -> Notification {
        Self {
            // Assigns a new random id when created
            uid: Id::new(rand::random::<u64>()),

            remaining_time: time,
            original_time: time,

            text: text.into(),
            kind,
        }
    }
}

pub fn show_notifications(
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

    // Don't bother if there's no notifications, and always repaint if there are (so they disappear)
    if notifs.len() == 0 { return; } else { ctx.request_repaint(); }

    let painter = ctx.layer_painter(LayerId::new(egui::Order::Tooltip, "notifications".into()));
    let mut last_pos: f32 = 0.0;

    // Manually paint everything (allows inputs to pass through the notifications)
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

        let galley = painter.layout_no_wrap(format!("({prefix}) {}", notif.text), FontId::default(), text_color);
        let text_size = galley.size();

        const MARGIN_UP: f32 = 1.0;
        const MARGIN_DOWN: f32 = 1.0;
        const MARGIN_LEFT: f32 = 2.0;
        const MARGIN_RIGHT: f32 = 5.0;

        let rect_start = Pos2::new(2.0, last_pos);
        let rect_end = Pos2::new(rect_start.x + text_size.x + MARGIN_LEFT + MARGIN_RIGHT, rect_start.y + text_size.y + MARGIN_UP + MARGIN_DOWN);
        let text_start = Pos2::new(rect_start.x + MARGIN_LEFT, rect_start.y + MARGIN_UP);

        painter.rect_filled(Rect::from_two_pos(rect_start, rect_end), 2.0, fill_color);
        painter.galley_with_color(text_start, galley, text_color);

        const SPACE_BETWEEN_RECTS: f32 = 3.0;

        last_pos += (rect_end.y - rect_start.y) + SPACE_BETWEEN_RECTS;
    }
}