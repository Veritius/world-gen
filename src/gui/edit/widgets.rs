use std::{ops::RangeInclusive, str::FromStr};

use eframe::{egui, emath::Numeric};
use crate::world::time::TimeLength;

fn parse_val(str: &str) -> Option<f64> {
    match TimeLength::from_str(str) {
        Ok(value) => {
            return Some(value.to_f64());
        },
        Err(_) => {
            return None;
        },
    }
}

pub fn time_length_drag_value(tl: &mut TimeLength) -> egui::DragValue {
    egui::DragValue::new(tl)
    .custom_formatter(|n, _| {
        format!("{}", TimeLength::from_f64(n))
    })
    .custom_parser(|str| {
        parse_val(str)
    })
}

pub fn time_length_slider(tl: &mut TimeLength, range: RangeInclusive<TimeLength>) -> egui::Slider {
    egui::Slider::new(tl, range)
    .custom_formatter(|n, _| {
        format!("{}", TimeLength::from_f64(n))
    })
    .custom_parser(|str| {
        parse_val(str)
    })
}