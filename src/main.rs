use eframe::{egui, run_native, App, CreationContext, Frame, NativeOptions};
use egui::emath::OrderedFloat;
use egui::load::{ImageLoader, ImagePoll, SizeHint};
use egui::widgets::Slider;
use egui::{Color32, TextureOptions, CentralPanel, ColorImage, Context};
use egui_extras::install_image_loaders;
use egui_extras::loaders::image_loader::ImageCrateLoader;
use std::sync::Arc;

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
    image: Option<Arc<ColorImage>>,
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

fn clamp(v: i32) -> u8 {
    v.clamp(0, 255) as u8
}

fn clamp_float(v: f32) -> usize {
    v.clamp(0.0, 255.0) as usize
}

fn change_pixel_brightness(p: Color32, brightness: i32) -> Color32 {
    Color32::from_rgb(
        clamp((p.r() as i32) + brightness),
        clamp((p.g() as i32) + brightness),
        clamp((p.b() as i32) + brightness),
    )
}

fn change_brightness(orig: &ColorImage, brightness: i32) -> ColorImage {
    let mut new_pixels = Vec::new();
    let orig_w = orig.size[0];
    let orig_h = orig.size[1];
    let num_orig_pixels = orig_w * orig_h;
    for i in 0..num_orig_pixels {
        new_pixels.push(change_pixel_brightness(orig.pixels[i], brightness));
    }
    ColorImage::new(orig.size, new_pixels)
}

fn histogram(img: &ColorImage) -> ColorImage {
    let mut buckets = [0; 256];
    let img_w = img.size[0];
    let img_h = img.size[1];
    let num_orig_pixels = img_w * img_h;
    for i in 0..num_orig_pixels {
        let p = img.pixels[i];
        let r = p.r() as f32;
        let g = p.g() as f32;
        let b = p.b() as f32;
        let avg = (r + g + b) / 3.0;
        buckets[clamp_float(avg)] += 1;
    }
    let max_count = *buckets.iter().max().unwrap();
    let w = 256;
    let h = 500;
    let mut new_pixels = Vec::new();
    for r in 0..h {
        for c in 0..w {
            let count_pixels = buckets[c];
            let percent_pixels = (count_pixels as f32) / (max_count as f32);
            let height_of_this_bar = ((h - r) as f32) / (h as f32);
            let v: u8 = if percent_pixels >= height_of_this_bar { 255 } else { 0 };
            new_pixels.push(Color32::from_rgb(v, v, v));
        }
    }
    ColorImage::new([w, h], new_pixels)
}

impl App for PhotoEditor {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        CentralPanel::default().show(ctx, |ui| {
            let scrolled = ctx.input(|state| state.raw_scroll_delta.y);

            if self.image == None {
                let image = Some(self.loader.load(
                    ctx,
                    r"file://C:\Users\justi\Pictures\CO_to_share\smaller\20251025_EvaLeafy.jpg",
                    SizeHint::Scale(OrderedFloat(1.0)),
                ));
                match image {
                    Some(Ok(ImagePoll::Ready {
                        image: color_image,
                    })) => {
                        self.image = Some(color_image);
                    },
                    Some(Err(er)) => println!("failed to load image because '{}'", er),
                    _ => (),
                }
            }
            let slider = Slider::new(&mut self.brightness, -255..=255).text("Brightness");
            if ui.add(slider).hovered() {
                self.brightness += sign(scrolled);
            }

            self.image.as_ref().map(|full_image| {
                let img = downsample(&full_image, 10);
                let img = change_brightness(&img, self.brightness);
                let hist_th = ctx.load_texture("histogram", histogram(&img), TextureOptions::LINEAR);
                let texture_handle = ctx.load_texture("preview", img, TextureOptions::LINEAR);
                ui.horizontal(|ui| {
                    ui.image((texture_handle.id(), texture_handle.size_vec2()));
                    ui.image((hist_th.id(), hist_th.size_vec2()));
                });
            });
        });
    }
}
