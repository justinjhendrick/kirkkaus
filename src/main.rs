use eframe::{egui, run_native, App, CreationContext, Frame, NativeOptions};
use egui::emath::OrderedFloat;
use egui::load::{Bytes, ImageLoadResult, ImageLoader, ImagePoll, SizeHint};
use egui::widgets::{Image, Slider};
use egui::TextureOptions;
use egui::{CentralPanel, ColorImage, Context};
use egui_extras::install_image_loaders;
use egui_extras::loaders::image_loader::ImageCrateLoader;
use log::{debug, error, info, log_enabled, Level};
use std::sync::Arc;
use std::task::Poll::Ready;

fn main() {
    env_logger::init();
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
    loader: ImageCrateLoader,
}

impl PhotoEditor {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            brightness: 0,
            image: None,
            loader: ImageCrateLoader::default(),
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

            self.image = Some(self.loader.load(
                ctx,
                r"file://C:\Users\justi\Pictures\CO_to_share\smaller\20251025_EvaLeafy.jpg",
                SizeHint::Scale(OrderedFloat(1.0)),
            ));
            match &self.image {
                Some(Ok(ImagePoll::Ready {
                    image: orig_color_image,
                })) => {
                    // https://github.com/emilk/egui/discussions/3431
                    let color_image = orig_color_image.clone(); // FIXME expensive copy
                    let handle = ctx.load_texture("eva_leafy", color_image, TextureOptions::LINEAR);
                    let sized_image = egui::load::SizedTexture::new(
                        handle.id(),
                        egui::vec2(
                            orig_color_image.size[0] as f32,
                            orig_color_image.size[1] as f32,
                        ),
                    );
                    let image = egui::Image::from_texture(sized_image);
                    ui.add(image);
                }
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
