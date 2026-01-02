use eframe::egui;
use eframe::{App, Frame};
use egui::widgets::Slider;
use egui::CentralPanel;
use egui::Context;

fn main() {
    let _ = eframe::run_native(
        "Kirkkaus",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(PhotoEditor::new(cc)))),
    );
}

struct PhotoEditor {
    brightness: i32,
}

impl PhotoEditor {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self { brightness: 0 }
    }
}

fn sign(i: f32) -> i32 {
    if i > 0.0 {
        1
    } else if i < 0.0 {
        -1
    } else {
        0
    }
}

impl App for PhotoEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let scrolled = ctx.input(|state| state.raw_scroll_delta.y);
            let slider = Slider::new(&mut self.brightness, -100..=100).text("Brightness");
            if ui.add(slider).hovered() {
                self.brightness += sign(scrolled);
            }
        });
    }
}
