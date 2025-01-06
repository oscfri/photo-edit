mod conversions;
mod types;
mod functions;

use crate::types::*;

use iced;
use image;
use num;

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
    temperature: f32
}

#[derive(Debug, Clone)]
enum Message {
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    ImageUpdated(iced::widget::image::Handle) // TODO: Maybe don't return a handle?
}
    
fn update_image(mut image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
    // NOTE: This takes ~160ms
    let now = std::time::SystemTime::now();
    functions::contrast(&mut image, parameters.contrast);
    functions::brightness(&mut image, parameters.brightness);
    functions::tint(&mut image, parameters.tint);
    functions::temperature(&mut image, parameters.temperature);
    let rgb_image: RgbImage = conversions::lab_image_to_rgb(&image);
    match now.elapsed() {
        Ok(elapsed) => {
            // it prints '2'
            println!("{}", elapsed.as_millis());
        }
        Err(e) => {
            // an error occurred!
            println!("Error: {e:?}");
        }
    }

    // NOTE: This takes ~80ms
    iced::widget::image::Handle::from_rgba(
        rgb_image.width as u32,
        rgb_image.height as u32,
        rgb_image_to_bytes(&rgb_image))
}

async fn update_image_async(image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
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
    (num::clamp(value, 0.0, 1.0) * 255.0) as u8
}

fn rgb_image_to_bytes(image: &RgbImage) -> iced::advanced::image::Bytes {
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

    iced::advanced::image::Bytes::from(buffer)
}

struct Main {
    source_image: LabImage,
    parameters: Parameters,
    handle: iced::widget::image::Handle,

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
        iced::widget::row![
                iced::widget::image(self.handle.clone()),
                self.view_sliders()
            ]
            .into()
    }
    
    fn view_sliders(&self) -> iced::Element<Message> {
        iced::widget::column![
                iced::widget::slider(-100.0..=100.0, self.parameters.brightness, Message::BrightnessChanged),
                iced::widget::slider(-100.0..=100.0, self.parameters.contrast, Message::ContrastChanged),
                iced::widget::slider(-100.0..=100.0, self.parameters.tint, Message::TintChanged),
                iced::widget::slider(-100.0..=100.0, self.parameters.temperature, Message::TemperatureChanged)
            ]
            .into()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}