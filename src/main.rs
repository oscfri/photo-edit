mod album;
mod conversions;
mod pixelwise;
mod types;
mod pipeline;

use album::{load_album, Album, AlbumImage, WorkImage};
use iced::{self, widget::container};
use native_dialog;
use types::RawImage;
use std::path::PathBuf;


pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run()
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    SetImage(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    ImageUpdated(RawImage),
    ImageMouseOver(iced::Point<f32>),
    ImageMousePress
}

struct Main {
    album: album::Album,
    image_index: usize,

    display_image: RawImage,
    mouse_position: iced::Point<f32>,

    // For synchronization
    updating_image: bool,
    needs_update: bool,

    viewport: pipeline::viewport::Viewport
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
        let display_image: RawImage = album.images[image_index]
            .into_work_image()
            .apply_parameters();
        let mouse_position: iced::Point<f32> = iced::Point {
            x: 0.0,
            y: 0.0
        };

        let updating_image: bool = false;
        let needs_update: bool = false;
        let viewport = pipeline::viewport::Viewport {
            image: display_image.clone(),
            image_index: image_index,
            parameters: album.images[image_index].parameters.clone()
        };

        Self {
            album,
            image_index,
            display_image,
            mouse_position,
            updating_image,
            needs_update,
            viewport
        }
    }
    
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        let should_update_image: bool = match message {
            Message::LoadAlbum => {
                let path: PathBuf = std::env::current_dir().unwrap();

                let result = native_dialog::FileDialog::new()
                    .set_location(&path)
                    .add_filter("image", &["png", "jpg"])
                    .show_open_multiple_file();

                match result {
                    Ok(file_paths) => {
                        self.album = load_album(&file_paths);
                        true
                    },
                    _ => {
                        false
                    }
                }
            },
            Message::NextImage => {
                self.image_index = (self.image_index + 1) % self.album.images.len();
                true
            },
            Message::SetImage(index) => {
                if index < self.album.images.len() {
                    self.image_index = index;
                    true
                } else {
                    false
                }
            },
            Message::BrightnessChanged(brightness) => {
                self.current_image_mut().parameters.brightness = brightness;
                true
            },
            Message::ContrastChanged(contrast) => {
                self.current_image_mut().parameters.contrast = contrast;
                true
            },
            Message::TintChanged(tint) => {
                self.current_image_mut().parameters.tint = tint;
                true
            },
            Message::TemperatureChanged(temperature) => {
                self.current_image_mut().parameters.temperature = temperature;
                true
            },
            Message::SaturationChanged(saturation) => {
                self.current_image_mut().parameters.saturation = saturation;
                true
            },
            Message::ImageUpdated(raw_image) => {
                self.viewport.image = raw_image.clone();
                self.viewport.image_index = self.image_index;
                self.viewport.parameters = self.current_image().parameters.clone();
                self.display_image = raw_image;
                self.updating_image = false;
                self.needs_update
            },
            Message::ImageMouseOver(point) => {
                self.mouse_position = point;
                false
            },
            Message::ImageMousePress => {
                // TODO: This doesn't really work. Mouse position doesn't necessarily need to correspond to the
                // pixel value. Will fix this when a custom image renderer is implemented.
                let x: usize = self.mouse_position.x as usize;
                let y: usize = self.mouse_position.y as usize;
                let current_image = self.current_image_mut();
                match current_image.pixel_at(x, y) {
                    Some(pixel) => {
                        current_image.parameters.tint = -pixel.tint;
                        current_image.parameters.temperature = -pixel.temperature;
                        true
                    },
                    None => {
                        false
                    }
                }
            }
        };

        if should_update_image {
            self.update_image_task()
        } else {
            iced::Task::none()
        }
    }
    
    fn update_image_task(&mut self) -> iced::Task<Message> {
        if !self.updating_image {
            self.updating_image = true;
            self.needs_update = false;
            let work_image: album::WorkImage = self.current_image().into_work_image();
            let future = update_image_async(work_image);
            iced::Task::perform(future, Message::ImageUpdated)
        } else {
            self.needs_update = true;
            iced::Task::none()
        }
    }
    
    fn view(&self) -> iced::Element<Message> {
        iced::widget::row![
                self.view_image(),
                self.view_sliders()
            ]
            .width(iced::Fill)
            .height(iced::Fill)
            .into()
    }

    fn view_image(&self) -> iced::Element<Message> {
        iced::widget::column![
                self.view_image_area(),
                self.view_debugger(),
                self.view_thumbnails()
            ]
            .into()
    }

    fn view_image_area(&self) -> iced::Element<Message> {
        let image_area = iced::widget::shader(&self.viewport)
            .width(iced::Fill)
            .height(iced::Fill);
        let image_mouse_area = iced::widget::mouse_area(image_area)
            .on_move(Message::ImageMouseOver)
            .on_right_press(Message::ImageMousePress);
        image_mouse_area.into()
        // iced::widget::container(image_mouse_area).into()
    }

    fn view_debugger(&self) -> iced::Element<Message> {
        iced::widget::container(iced::widget::text(format!("{}", self.mouse_position)))
            .style(iced::widget::container::dark)
            .width(iced::Fill)
            .into()
    }

    fn view_thumbnails(&self) -> iced::Element<Message> {
        let thumbnails = self.album.images.iter().enumerate()
            .map(|(index, album_image)| self.view_thumbnail_image(index, &album_image))
            .collect();

        let row = iced::widget::Row::from_vec(thumbnails)
            .spacing(10);

        iced::widget::container(row)
            .padding(10)
            .height(120)
            .into()
    }

    fn view_thumbnail_image(&self, index: usize, album_image: &AlbumImage) -> iced::Element<Message> {
        let image_handle = iced::widget::image::Handle::from_rgba(
            album_image.thumbnail.width as u32,
            album_image.thumbnail.height as u32,
            album_image.thumbnail.pixels.clone());
        iced::widget::mouse_area(iced::widget::image(image_handle))
            .on_press(Message::SetImage(index))
            .into()
    }
    
    fn view_sliders(&self) -> iced::Element<Message> {
        let parameters: &album::Parameters = &self.current_image().parameters;
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
            .width(300)
            .height(iced::Fill)
            .style(iced::widget::container::bordered_box)
            .into()
    }

    fn current_image(&self) -> &album::AlbumImage {
        &self.album.images[self.image_index]
    }

    fn current_image_mut(&mut self) -> &mut album::AlbumImage {
        &mut self.album.images[self.image_index]
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}