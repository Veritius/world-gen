use eframe::{egui, emath::Numeric};
use crate::world::time::TimeLength;

pub fn time_length_drag_value(tl: &mut TimeLength) -> egui::DragValue {
    egui::DragValue::new(tl)
    .custom_formatter(|n, _| {
        format!("{}", TimeLength::from_f64(n))
    })
    .custom_parser(|_str| {
        Some(0.0)
    })
}