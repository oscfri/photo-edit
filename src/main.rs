mod conversions;
mod types;
mod functions;

use crate::types::*;

use conversions::{Converter, create_converter};
use iced;
use image;
use num;

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

struct Main {
    converter: Converter,
    source_image: LabImage,
    parameters: Parameters,
    handle: iced::widget::image::Handle
}
    
fn update_image(converter: &Converter, mut image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
    functions::brightness(&mut image, parameters.brightness);
    functions::contrast(&mut image, parameters.contrast);
    functions::tint(&mut image, parameters.tint);
    functions::temperature(&mut image, parameters.temperature);
    let rgb_image: RgbImage = converter.lab_image_to_rgb(&image);
    iced::widget::image::Handle::from_rgba(
        rgb_image.width as u32,
        rgb_image.height as u32,
        rgb_image_to_bytes(&rgb_image))
}

async fn update_image_async(converter: Converter, image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
    update_image(&converter, image, parameters)
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

fn load_image_as_lab(converter: &Converter) -> LabImage {
    let image: RgbImage = load_image();
    converter.rgb_image_to_lab(&image)
}

fn pixel_value_to_u8(value: f32) -> u8 {
    (num::clamp(value, 0.0, 1.0) * 255.0) as u8
}

fn rgb_image_to_bytes(image: &RgbImage) -> iced::advanced::image::Bytes {
    let mut buffer: Vec<u8> = vec![0; image.width * image.height * 4];

    for (index, pixel) in image.pixels.iter().enumerate() {
        buffer[index * 4 + 0] = pixel_value_to_u8(pixel.red);
        buffer[index * 4 + 1] = pixel_value_to_u8(pixel.green);
        buffer[index * 4 + 2] = pixel_value_to_u8(pixel.blue);
        buffer[index * 4 + 3] = 255; // Alpha
    }

    iced::advanced::image::Bytes::from(buffer)
}

impl Main {

    fn new() -> Self {
        let converter: Converter = create_converter();
        let source_image: LabImage = load_image_as_lab(&converter);
        let parameters: Parameters = Parameters::default();
    
        let handle = update_image(&converter, source_image.clone(), parameters);

        Self {
            converter,
            source_image,
            parameters,
            handle
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
                iced::Task::none()
            }
        }
    }
    
    fn update_image_task(&self) -> iced::Task<Message> {
        iced::Task::perform(update_image_async(self.converter.clone(), self.source_image.clone(), self.parameters), Message::ImageUpdated)
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