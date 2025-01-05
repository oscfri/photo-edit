mod conversions;
mod types;
mod functions;

use crate::types::*;

use iced;
use image;
use num;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", update, view)
        .theme(|_| iced::Theme::Dark)
        .resizable(true)
        .run_with(initialize)
}

fn initialize() -> (State, iced::Task<Message>) {
    let source_image: LabImage = load_image_as_lab();
    let parameters: Parameters = Parameters::default();

    let handle = update_image(source_image.clone(), parameters);

    let state: State = State {
        source_image: source_image,
        parameters: Parameters::default(),
        handle: handle
    };
    (state, iced::Task::none())
}

#[derive(Default, Clone, Copy)]
struct Parameters {
    brightness: f32,
    contrast: f32,
    tint: f32,
    temperature: f32
}

struct State {
    source_image: LabImage,
    parameters: Parameters,
    handle: iced::widget::image::Handle
}

#[derive(Debug, Clone)]
enum Message {
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    ImageUpdated(iced::widget::image::Handle) // TODO: Maybe don't return a handle?
}

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::BrightnessChanged(brightness) => {
            state.parameters.brightness = brightness;
            update_image_task(&state)
        },
        Message::ContrastChanged(contrast) => {
            state.parameters.contrast = contrast;
            update_image_task(&state)
        },
        Message::TintChanged(tint) => {
            state.parameters.tint = tint;
            update_image_task(&state)
        },
        Message::TemperatureChanged(temperature) => {
            state.parameters.temperature = temperature;
            update_image_task(&state)
        },
        Message::ImageUpdated(handle) => {
            state.handle = handle;
            iced::Task::none()
        }
    }
}

fn update_image(mut image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
    functions::brightness(&mut image, parameters.brightness);
    functions::contrast(&mut image, parameters.contrast);
    functions::tint(&mut image, parameters.tint);
    functions::temperature(&mut image, parameters.temperature);
    let rgb_image: RgbImage = conversions::lab_image_to_rgb(&image);
    iced::widget::image::Handle::from_rgba(
        rgb_image.width as u32,
        rgb_image.height as u32,
        rgb_image_to_bytes(&rgb_image))
}

async fn update_image_async(image: LabImage, parameters: Parameters) -> iced::widget::image::Handle {
    update_image(image, parameters)
}

fn update_image_task(state: &State) -> iced::Task<Message> {
    iced::Task::perform(update_image_async(state.source_image.clone(), state.parameters), Message::ImageUpdated)
}

fn view(state: &State) -> iced::Element<Message> {
    iced::widget::row![
            iced::widget::image(state.handle.clone()),
            view_sliders(&state)
        ]
        .into()
}

fn view_sliders(state: &State) -> iced::Element<Message> {
    iced::widget::column![
            iced::widget::slider(-100.0..=100.0, state.parameters.brightness, Message::BrightnessChanged),
            iced::widget::slider(-100.0..=100.0, state.parameters.contrast, Message::ContrastChanged),
            iced::widget::slider(-100.0..=100.0, state.parameters.tint, Message::TintChanged),
            iced::widget::slider(-100.0..=100.0, state.parameters.temperature, Message::TemperatureChanged)
        ]
        .into()
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
    let mut buffer: Vec<u8> = vec![0; image.width * image.height * 4];

    for (index, pixel) in image.pixels.iter().enumerate() {
        buffer[index * 4 + 0] = pixel_value_to_u8(pixel.red);
        buffer[index * 4 + 1] = pixel_value_to_u8(pixel.green);
        buffer[index * 4 + 2] = pixel_value_to_u8(pixel.blue);
        buffer[index * 4 + 3] = 255; // Alpha
    }

    iced::advanced::image::Bytes::from(buffer)
}