//! Big windows that can only be removed by confirmation and block interaction behind them.

use std::collections::BTreeSet;
use eframe::{epaint::{Color32, Stroke, Vec2, FontId}, egui::{self, Margin}};

pub struct ModalWindow {
    outline_color: Color32,
    internal_color: Color32,
    text: String,
}

impl ModalWindow {
    pub(super) fn display(&self, ctx: &egui::Context, markers: &mut BTreeSet<String>) { 
        // Figure out how much space to give to the frame
        let mut frame_size: [f32; 2] = [0.0; 2];
        ctx.fonts(|i| {
            let text_size = i.layout_no_wrap(self.text.clone(), FontId::default(), Color32::DEBUG_COLOR).rect;
            frame_size = [text_size.width(), text_size.height()];
        });
        frame_size[0] += 30.0;
        frame_size[1] += 20.0;

        // The frame for the window
        let frame = egui::Frame::none()
        .fill(self.internal_color)
        .stroke(Stroke::new(2.0, self.outline_color))
        .rounding(5.0)
        .inner_margin(Margin::symmetric(10.0, 10.0));
    
        // Show window
        egui::Window::new("modal")
        .anchor(eframe::emath::Align2::CENTER_CENTER, [0.0; 2])
        .fixed_size(frame_size) // take up the entire window so clicks don't get through
        .resizable(false)
        .collapsible(false)
        .title_bar(false)
        .scroll2([false; 2])
        .frame(frame) // no background
        .show(ctx, |ui| {
            ui.centered_and_justified(|ui| {
                ui.label(self.text.clone());
                ui.add_space(2.0);

                if ui.button("Ok").clicked() {
                    markers.insert("remove_modal".to_string());
                }
            });
        });
    }

    pub fn new(text: impl Into<String>) -> ModalWindow {
        ModalWindow {
            outline_color: Color32::LIGHT_BLUE,
            internal_color: Color32::from_rgb(50, 50, 50),
            text: text.into()
        }
    }

    pub fn outline_color(mut self, color: Color32) -> Self {
        self.outline_color = color;
        self
    }

    pub fn internal_color(mut self, color: Color32) -> Self {
        self.internal_color = color;
        self
    }
}