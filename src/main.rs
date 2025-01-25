mod album;
mod types;
mod pipeline;
mod workspace;

use album::{Album, AlbumImage};
use iced::{self, widget::container};
use native_dialog;
use workspace::WorkSpace;
use std::path::PathBuf;


pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run()
}

enum ImageMouseMessage {
    Over(iced::Point<f32>),
    Press,
    Release
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    EnterCropMode,
    SetImage(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    ImageMouseOver(iced::Point<f32>),
    ImageMousePress
}

enum Mode {
    Normal,
    Crop
}

struct Main {
    workspace: WorkSpace,

    mouse_position: iced::Point<f32>,
    mode: Mode,

    viewport: pipeline::viewport::Viewport
}

fn load_workspace(file_paths: &Vec<PathBuf>) -> WorkSpace {
    let album: Album = album::load_album(file_paths);
    let image_index: usize = 0;
    WorkSpace::new(album, image_index)
}

impl Main {

    fn new() -> Self {
        let workspace: WorkSpace = load_workspace(&vec![
            PathBuf::from("example.png"),
            PathBuf::from("example2.jpg")
        ]);

        let mouse_position: iced::Point<f32> = iced::Point {
            x: 0.0,
            y: 0.0
        };
        let viewport = workspace.make_viewport();

        let mode: Mode = Mode::Normal;

        Self {
            workspace,
            mouse_position,
            mode,
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
                        self.workspace = load_workspace(&file_paths);
                        true
                    },
                    _ => {
                        false
                    }
                }
            },
            Message::NextImage => {
                self.workspace.next_image_index();
                true
            },
            Message::SetImage(index) => {
                self.workspace.set_image_index(index);
                true
            },
            Message::EnterCropMode => {
                self.mode = Mode::Crop;
                false
            }
            Message::BrightnessChanged(brightness) => {
                self.workspace.current_parameters_mut().brightness = brightness;
                true
            },
            Message::ContrastChanged(contrast) => {
                self.workspace.current_parameters_mut().contrast = contrast;
                true
            },
            Message::TintChanged(tint) => {
                self.workspace.current_parameters_mut().tint = tint;
                true
            },
            Message::TemperatureChanged(temperature) => {
                self.workspace.current_parameters_mut().temperature = temperature;
                true
            },
            Message::SaturationChanged(saturation) => {
                self.workspace.current_parameters_mut().saturation = saturation;
                true
            },
            Message::ImageMouseOver(point) => {
                self.mouse_position = point;
                false
            },
            Message::ImageMousePress => {
                // TODO: This doesn't really work. Mouse position doesn't necessarily need to correspond to the
                // pixel value. Will fix this when a custom image renderer is implemented.
                false
                // TODO: Reimplement this
                // let x: usize = self.mouse_position.x as usize;
                // let y: usize = self.mouse_position.y as usize;
                // let current_image = self.current_image_mut();
                // match current_image.pixel_at(x, y) {
                //     Some(pixel) => {
                //         current_image.parameters.tint = -pixel.tint;
                //         current_image.parameters.temperature = -pixel.temperature;
                //         true
                //     },
                //     None => {
                //         false
                //     }
                // }
            }
        };

        if should_update_image {
            self.update_image_task()
        } else {
            iced::Task::none()
        }
    }
    
    fn update_image_task(&mut self) -> iced::Task<Message> {
        self.viewport = self.workspace.make_viewport();
        iced::Task::none()
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
        let thumbnails = self.workspace.album_images().iter().enumerate()
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
        let parameters: &album::Parameters = self.workspace.current_parameters();
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
                iced::widget::button("Crop").on_press(Message::EnterCropMode),
            ];
        container(column)
            .padding(10)
            .width(300)
            .height(iced::Fill)
            .style(iced::widget::container::bordered_box)
            .into()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}