use eframe::{egui, run_native, App, CreationContext, Frame, NativeOptions};
use egui::emath::OrderedFloat;
use egui::load::{Bytes, ImageLoadResult, ImageLoader, ImagePoll, SizeHint};
use egui::widgets::{Image, Slider};
use egui::{Color32, TextureOptions, TextureHandle, CentralPanel, ColorImage, Context};
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
    texture_handle: Option<TextureHandle>,
    image: Option<ColorImage>,
    loader: ImageCrateLoader,
}

impl PhotoEditor {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            brightness: 0,
            texture_handle: None,
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

fn downsample(orig: &ColorImage, skip_factor: usize) -> ColorImage {
    let mut new_pixels = Vec::new();
    let orig_w = orig.size[0];
    let orig_h = orig.size[1];
    let num_orig_pixels = orig_w * orig_h;
    for i in 0..num_orig_pixels {
        let r = i / orig_w;
        let c = i % orig_w;
        if r % skip_factor == 0 && c % skip_factor == 0 {
            new_pixels.push(orig.pixels[i]);
        }
    }
    ColorImage::new([orig_w / skip_factor, orig_h / skip_factor], new_pixels)
}

fn change_brightness(orig: &ColorImage, brightness: i32) -> ColorImage {
    let mut new_pixels = Vec::new();
    let orig_w = orig.size[0];
    let orig_h = orig.size[1];
    let num_orig_pixels = orig_w * orig_h;
    let b = brightness as u8; // FIXME
    for i in 0..num_orig_pixels {
        let p = orig.pixels[i];
        let new_p = Color32::from_rgb(p.r() + b, p.g() + b, p.b() + b);
        new_pixels.push(new_p);
    }
    ColorImage::new(orig.size, new_pixels)
}

impl App for PhotoEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let scrolled = ctx.input(|state| state.raw_scroll_delta.y);

            if self.texture_handle == None {
                let image = Some(self.loader.load(
                    ctx,
                    r"file://C:\Users\justi\Pictures\CO_to_share\smaller\20251025_EvaLeafy.jpg",
                    SizeHint::Scale(OrderedFloat(1.0)),
                ));
                match image {
                    Some(Ok(ImagePoll::Ready {
                        image: color_image,
                    })) => {
                        let im = downsample(&color_image, 10);
                        self.image = Some(im.clone());
                        self.texture_handle = Some(ctx.load_texture("eva_leafy", im, TextureOptions::LINEAR));
                    }
                    Some(Err(er)) => println!("failed to load image because '{}'", er),
                    _ => (),
                }
            }
            self.texture_handle.as_ref().map(|th| {
                ui.image((th.id(), th.size_vec2()));
            });

            let slider = Slider::new(&mut self.brightness, -100..=100).text("Brightness");
            if ui.add(slider).hovered() {
                self.brightness += sign(scrolled);
            }
            self.image.clone().map(|im| {
                self.image = Some(change_brightness(&im, self.brightness));
            });
        });
    }
}
