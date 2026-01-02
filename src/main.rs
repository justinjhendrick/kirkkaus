use eframe::{egui, run_native, App, CreationContext, Frame, NativeOptions};
use egui::emath::OrderedFloat;
use egui::load::{Bytes, ImageLoadResult, ImageLoader, ImagePoll, SizeHint};
use egui::widgets::{Image, Slider};
use egui::{CentralPanel, ColorImage, Context};
use egui_extras::install_image_loaders;
use egui_extras::loaders::image_loader::ImageCrateLoader;
use std::sync::Arc;
use std::task::Poll::Ready;

fn main() {
    let _ = run_native(
        "Kirkkaus",
        NativeOptions::default(),
        Box::new(|cc| {
            install_image_loaders(&cc.egui_ctx);
            Ok(Box::new(PhotoEditor::new(cc)))
        }),
    );
}

struct PhotoEditor {
    brightness: i32,
    image: Option<ImageLoadResult>,
}

impl PhotoEditor {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            brightness: 0,
            image: None, //Image::from_uri(
        }
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

            self.image = Some(ImageCrateLoader::default().load(
                ctx,
                "file:///home/justin/pebble-one-handed-minute/screenshot_emery.png",
                SizeHint::Scale(OrderedFloat(1.0)),
            ));
            match &self.image {
                Some(Ok(ImagePoll::Ready { image: color_image })) => {
                    println!("finished image load, displaying"); // FIXME why doesn' code reach here?
                    let image = Image::from_bytes(
                        "bytes://screenshot_emery",
                        Bytes::Shared(Arc::from(color_image.as_raw().to_vec())), // FIXME expensive copy maybe?
                    );
                    ui.add(image);
                }
                Some(Ok(ImagePoll::Pending { size: _ })) => println!("image is pending"), // FIXME always pending!
                Some(Err(er)) => println!("failed to load image because '{}'", er),
                _ => (),
            }

            let slider = Slider::new(&mut self.brightness, -100..=100).text("Brightness");
            if ui.add(slider).hovered() {
                self.brightness += sign(scrolled);
            }
        });
    }
}
