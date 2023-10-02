use std::ops::RangeInclusive;
use bevy_egui::egui::{emath::Numeric, Ui, Slider};

pub fn titled_text(
    ui: &mut Ui,
    title: impl Into<String>,
    value: &mut String,
) {
    ui.horizontal(|ui| {
        ui.label(title.into());
        ui.text_edit_singleline(value);
    });
}

pub fn titled_slider<T: PartialOrd + Numeric>(
    ui: &mut Ui,
    title: impl Into<String>,
    value: &mut T,
    range: RangeInclusive<T>
) {
    ui.horizontal(|ui| {
        ui.label(title.into());
        ui.add(Slider::new(value, range));
    });
}