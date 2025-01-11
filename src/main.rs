mod conversions;
mod types;
mod pixelwise;

use crate::types::*;

use iced::{self, widget::container};
use image;

use rayon::prelude::*;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Dark)
        .resizable(true)
        .run()
}
    
#[derive(Default, Clone, Copy)]
struct Parameters {
    brightness: f32,
    contrast: f32,
    tint: f32,
    temperature: f32,
    saturation: f32
}

#[derive(Debug, Clone)]
enum Message {
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    ImageUpdated(Vec<u8>)
}
    
fn update_image(mut image: LabImage, parameters: Parameters) -> Vec<u8> {
    // NOTE: This takes ~30ms
    pixelwise::contrast(&mut image, parameters.contrast);
    pixelwise::brightness(&mut image, parameters.brightness);
    pixelwise::saturation(&mut image, parameters.saturation);
    pixelwise::tint(&mut image, parameters.tint);
    pixelwise::temperature(&mut image, parameters.temperature);
    // NOTE: This takes ~70ms
    let rgb_image: RgbImage = conversions::lab_image_to_rgb(&image);

    // NOTE: This takes ~80ms
    rgb_image_to_bytes(&rgb_image)
}

async fn update_image_async(image: LabImage, parameters: Parameters) -> Vec<u8> {
    update_image(image, parameters)
}
    
fn load_image() -> RgbImage {
    let source_image = image::open("example.png").unwrap().into_rgb32f();
    let width: u32 = source_image.width();
    let height: u32 = source_image.height();
    let size = width * height;
    let mut pixels: Vec<RgbPixel> = Vec::with_capacity(size as usize);
    for h in 0..height {
        for w in 0..width {
            let rgb = source_image.get_pixel(w, h);
            pixels.push(RgbPixel {
                red: rgb[0],
                green: rgb[1],
                blue: rgb[2]
            });
        }
    }
    RgbImage {
        width: width as usize,
        height: height as usize,
        pixels: pixels
    }
}

fn load_image_as_lab() -> LabImage {
    let image: RgbImage = load_image();
    conversions::rgb_image_to_lab(&image)
}

fn pixel_value_to_u8(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (value * 255.0) as u8
    }
}

fn rgb_image_to_bytes(image: &RgbImage) -> Vec<u8> {
    let mut buffer: Vec<u8> = vec![255; image.width * image.height * 4];

    buffer.par_iter_mut()
        .enumerate()
        .for_each(|(index, byte)| {
            let pixel_index: usize = index / 4;
            let channel_index: usize = index % 4;
            if channel_index == 0 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].red);
            } else if channel_index == 1 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].green);
            } else if channel_index == 2 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].blue);
            }
            // Don't bother with alpha, as it's 255 by default
        });

    buffer
}

struct Main {
    source_image: LabImage,
    parameters: Parameters,
    handle: Vec<u8>,

    // For synchronization
    updating_image: bool,
    needs_update: bool
}

impl Main {

    fn new() -> Self {
        let source_image: LabImage = load_image_as_lab();
        let parameters: Parameters = Parameters::default();
        let updating_image: bool = false;
        let needs_update: bool = false;
    
        let handle = update_image(source_image.clone(), parameters);

        Self {
            source_image,
            parameters,
            handle,
            updating_image,
            needs_update
        }
    }
    
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::BrightnessChanged(brightness) => {
                self.parameters.brightness = brightness;
                self.update_image_task()
            },
            Message::ContrastChanged(contrast) => {
                self.parameters.contrast = contrast;
                self.update_image_task()
            },
            Message::TintChanged(tint) => {
                self.parameters.tint = tint;
                self.update_image_task()
            },
            Message::TemperatureChanged(temperature) => {
                self.parameters.temperature = temperature;
                self.update_image_task()
            },
            Message::SaturationChanged(saturation) => {
                self.parameters.saturation = saturation;
                self.update_image_task()
            },
            Message::ImageUpdated(handle) => {
                self.handle = handle;
                self.updating_image = false;
                if self.needs_update {
                    self.update_image_task()
                } else {
                    iced::Task::none()
                }
            }
        }
    }
    
    fn update_image_task(&mut self) -> iced::Task<Message> {
        if !self.updating_image {
            self.updating_image = true;
            self.needs_update = false;
            let future = update_image_async(self.source_image.clone(), self.parameters);
            iced::Task::perform(future, Message::ImageUpdated)
        } else {
            self.needs_update = true;
            iced::Task::none()
        }
    }
    
    fn view(&self) -> iced::Element<Message> {
        let handle = iced::widget::image::Handle::from_rgba(
            self.source_image.width as u32,
            self.source_image.height as u32,
            self.handle.clone());
        iced::widget::row![
                iced::widget::image(handle),
                self.view_sliders()
            ]
            .into()
    }
    
    fn view_sliders(&self) -> iced::Element<Message> {
        let column = iced::widget::column![
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, Message::BrightnessChanged),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, Message::ContrastChanged),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint, Message::TintChanged),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, Message::TemperatureChanged),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, self.parameters.saturation, Message::SaturationChanged),
            ];
        container(column)
            .padding(10)
            .into()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}