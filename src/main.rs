mod album;
mod conversions;
mod pixelwise;
mod types;

use album::{load_album, Album, WorkImage};
use iced::{self, widget::container};
use native_dialog;
use types::RawImage;
use std::path::PathBuf;


pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Dark)
        .resizable(true)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    ImageUpdated(RawImage)
}

struct Main {
    album: album::Album,
    image_index: usize,

    display_image: RawImage,

    // For synchronization
    updating_image: bool,
    needs_update: bool
}

async fn update_image_async(work_image: WorkImage) -> RawImage {
    work_image.apply_parameters()
}

impl Main {

    fn new() -> Self {
        let album: Album = album::load_album(&vec![
            PathBuf::from("example.png"),
            PathBuf::from("example2.jpg")
        ]);
        let image_index: usize = 0;
        let updating_image: bool = false;
        let needs_update: bool = false;
    
        let display_image: RawImage = album.images[image_index].clone().apply_parameters();

        Self {
            album,
            image_index,
            display_image,
            updating_image,
            needs_update
        }
    }
    
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::LoadAlbum => {
                let path: PathBuf = std::env::current_dir().unwrap();

                let result = native_dialog::FileDialog::new()
                    .set_location(&path)
                    .add_filter("image", &["png", "jpg"])
                    .show_open_multiple_file();

                match result {
                    Ok(file_paths) => {
                        self.album = load_album(&file_paths);
                        self.update_image_task()
                    },
                    _ => {
                        iced::Task::none()
                    }
                }
            },
            Message::NextImage => {
                self.image_index = (self.image_index + 1) % self.album.images.len();
                self.update_image_task()
            },
            Message::BrightnessChanged(brightness) => {
                self.current_image_mut().parameters.brightness = brightness;
                self.update_image_task()
            },
            Message::ContrastChanged(contrast) => {
                self.current_image_mut().parameters.contrast = contrast;
                self.update_image_task()
            },
            Message::TintChanged(tint) => {
                self.current_image_mut().parameters.tint = tint;
                self.update_image_task()
            },
            Message::TemperatureChanged(temperature) => {
                self.current_image_mut().parameters.temperature = temperature;
                self.update_image_task()
            },
            Message::SaturationChanged(saturation) => {
                self.current_image_mut().parameters.saturation = saturation;
                self.update_image_task()
            },
            Message::ImageUpdated(raw_image) => {
                self.display_image = raw_image;
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
            let work_image: album::WorkImage = self.current_image().clone();
            let future = update_image_async(work_image);
            iced::Task::perform(future, Message::ImageUpdated)
        } else {
            self.needs_update = true;
            iced::Task::none()
        }
    }
    
    fn view(&self) -> iced::Element<Message> {
        let handle = iced::widget::image::Handle::from_rgba(
            self.display_image.width as u32,
            self.display_image.height as u32,
            self.display_image.pixels.clone());
        iced::widget::row![
                iced::widget::image::viewer(handle).width(800),
                self.view_sliders()
            ]
            .into()
    }
    
    fn view_sliders(&self) -> iced::Element<Message> {
        let parameters: album::Parameters = self.current_image().parameters;
        let column = iced::widget::column![
                iced::widget::button("Load").on_press(Message::LoadAlbum),
                iced::widget::text("Brightness"),
                iced::widget::slider(-100.0..=100.0, parameters.brightness, Message::BrightnessChanged),
                iced::widget::text("Contrast"),
                iced::widget::slider(-100.0..=100.0, parameters.contrast, Message::ContrastChanged),
                iced::widget::text("Tint"),
                iced::widget::slider(-100.0..=100.0, parameters.tint, Message::TintChanged),
                iced::widget::text("Temperature"),
                iced::widget::slider(-100.0..=100.0, parameters.temperature, Message::TemperatureChanged),
                iced::widget::text("Saturation"),
                iced::widget::slider(-100.0..=100.0, parameters.saturation, Message::SaturationChanged),
                iced::widget::button("Next").on_press(Message::NextImage),
            ];
        container(column)
            .padding(10)
            .into()
    }

    fn current_image(&self) -> &album::WorkImage {
        &self.album.images[self.image_index]
    }

    fn current_image_mut(&mut self) -> &mut album::WorkImage {
        &mut self.album.images[self.image_index]
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}