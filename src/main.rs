mod application;
mod conversions;
mod types;
mod functions;

use crate::types::*;

use iced;
use image;
use num;

// fn main() {
//     let mut image: LabImage = load_image_as_lab();

//     functions::brightness(&mut image, 10.0);
//     functions::contrast(&mut image, 1.1);
//     functions::tint(&mut image, -10.0);
//     functions::temperature(&mut image, 20.0);

//     let output_image: RgbImage = conversions::lab_image_to_rgb(&image);
//     draw_image(&output_image);
// }

pub fn main() -> iced::Result {
    iced::application("A cool image editor", update, view)
        .theme(|_| iced::Theme::Dark)
        .resizable(true)
        .run_with(initialize)
}

fn initialize() -> (State, iced::Task<Message>) {
    let source_image: LabImage = load_image_as_lab();
    let parameters: Parameters = Parameters::default();

    let handle = update_image(&source_image, &parameters);

    let state: State = State {
        source_image: source_image,
        parameters: Parameters::default(),
        handle: handle
    };
    (state, iced::Task::none())
}

#[derive(Default)]
struct Parameters {
    temperature: f32
}

struct State {
    source_image: LabImage,
    parameters: Parameters,
    handle: iced::widget::image::Handle
}

fn update(state: &mut State, message: Message) -> iced::Task<Message> {
    match message {
        Message::TemperatureChanged(temperature) => {
            state.parameters.temperature = temperature;
            iced::Task::perform(
                update_image(&state.source_image, &state.parameters),
                Message::ImageUpdated
            )
        },
        Message::ImageUpdated(handle) => {
            state.handle = handle;
            iced::Task::none()
        }
    }
}

fn update_image(source_image: &LabImage, parameters: &Parameters) -> iced::widget::image::Handle {
    let mut image: LabImage = source_image.clone();
    functions::temperature(&mut image, parameters.temperature);
    let rgb_image: RgbImage = conversions::lab_image_to_rgb(&image);
    iced::widget::image::Handle::from_rgba(
        rgb_image.width as u32,
        rgb_image.height as u32,
        rgb_image_to_bytes(&rgb_image))
}

#[derive(Debug, Clone)]
enum Message {
    TemperatureChanged(f32),
    ImageUpdated(iced::widget::image::Handle)
}

fn view(state: &State) -> iced::Element<Message> {
    // iced::widget::button("Load").on_press(Message::Load).into()
    iced::widget::row![
            iced::widget::image(state.handle.clone()),
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