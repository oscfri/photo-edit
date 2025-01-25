mod album;
mod types;
mod pipeline;
mod workspace;
mod view_mode;

use album::{AlbumImage, Crop};
use iced::{self, widget::container};
use native_dialog;
use pipeline::viewport;
use types::RawImage;
use view_mode::ViewMode;
use workspace::WorkSpace;
use std::path::PathBuf;

pub fn main() -> iced::Result {
    iced::application("A cool image editor", Main::update, Main::view)
        .theme(|_| iced::Theme::Nord)
        .resizable(true)
        .run()
}

#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Clone)]
enum MouseState {
    Up,
    Down
}

#[derive(Debug, Clone)]
enum MouseMessage {
    Over(Point),
    Press,
    Release
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    ToggleCropMode,
    SetImage(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    ImageMouseMessage(MouseMessage),
}

struct Main {
    workspace: WorkSpace,

    mouse_position: Point,
    view_mode: ViewMode,
    mouse_state: MouseState,

    viewport: viewport::Viewport
}

fn make_viewport(workspace: &WorkSpace, view_mode: &view_mode::ViewMode) -> viewport::Viewport {
    viewport::Viewport::new(workspace.make_viewport(), view_mode.clone())
}

impl Main {

    fn new() -> Self {
        let workspace: WorkSpace = workspace::load_workspace(&vec![
            PathBuf::from("example.png"),
            PathBuf::from("example2.jpg")
        ]);

        let mouse_position: Point = Point {
            x: 0,
            y: 0
        };
        let mode: view_mode::ViewMode = view_mode::ViewMode::Normal;
        let viewport = make_viewport(&workspace, &mode);
        let mouse_state: MouseState = MouseState::Up;

        Self {
            workspace,
            mouse_position,
            view_mode: mode,
            mouse_state,
            viewport
        }
    }
    
    fn update(&mut self, message: Message) -> iced::Task<Message> {
        let should_update_image: bool = match message {
            Message::LoadAlbum => {
                self.open_file_dialog()
            },
            Message::NextImage => {
                self.workspace.next_image_index();
                true
            },
            Message::SetImage(index) => {
                self.workspace.set_image_index(index);
                true
            },
            Message::ToggleCropMode => {
                if !matches!(self.view_mode, ViewMode::Crop) {
                    self.view_mode = ViewMode::Crop;
                } else {
                    self.view_mode = ViewMode::Normal;
                }
                true
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
            Message::ImageMouseMessage(image_mouse_message) => {
                self.update_mouse_on_image(image_mouse_message)
            },
            // Message::ImageMousePress => {
                // TODO: This doesn't really work. Mouse position doesn't necessarily need to correspond to the
                // pixel value. Will fix this when a custom image renderer is implemented.
                // false
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
            // }
        };

        if should_update_image {
            self.update_image_task();
        }

        iced::Task::none()
    }

    fn open_file_dialog(&mut self) -> bool {
        let path: PathBuf = std::env::current_dir().unwrap();

        let result = native_dialog::FileDialog::new()
            .set_location(&path)
            .add_filter("image", &["png", "jpg"])
            .show_open_multiple_file();

        match result {
            Ok(file_paths) => {
                self.workspace = workspace::load_workspace(&file_paths);
                true
            },
            _ => {
                false
            }
        }
    }

    fn update_mouse_on_image(&mut self, image_mouse_message: MouseMessage) -> bool {
        match image_mouse_message {
            MouseMessage::Over(point) => {
                self.mouse_position = point;
            },
            MouseMessage::Press => {
                self.mouse_state = MouseState::Down;
            },
            MouseMessage::Release => {
                self.mouse_state = MouseState::Up;
            }
        }
        
        match self.view_mode {
            ViewMode::Normal => {
                false
            },
            ViewMode::Crop => {
                self.update_mouse_crop_mode(image_mouse_message)
            }
        }
    }

    fn update_mouse_crop_mode(&mut self, image_mouse_message: MouseMessage) -> bool {
        match image_mouse_message {
            MouseMessage::Over(point) => {
                match self.mouse_state {
                    MouseState::Up => {
                        false
                    },
                    MouseState::Down => {
                        let crop: &mut Crop = self.workspace.current_crop_mut();
                        crop.x2 = point.x;
                        crop.y2 = point.y;
                        true
                    }
                }
            },
            MouseMessage::Press => {
                let crop: &mut Crop = self.workspace.current_crop_mut();
                crop.x1 = self.mouse_position.x;
                crop.y1 = self.mouse_position.y;
                crop.x2 = self.mouse_position.x;
                crop.y2 = self.mouse_position.y;
                true
            },
            MouseMessage::Release => {
                false
            }
        }
    }
    
    fn update_image_task(&mut self) {
        self.viewport = make_viewport(&self.workspace, &self.view_mode);
    }

    fn window_space_to_image_space(&self, point: iced::Point<f32>) -> Point {
        let current_image: &RawImage = self.workspace.current_image();
        Point {
            x: (point.x / viewport::get_viewport_width() * (current_image.width as f32)) as i32,
            y: (point.y / viewport::get_viewport_height() * (current_image.height as f32)) as i32,
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
            .on_move(|window_point| {
                let image_point: Point = self.window_space_to_image_space(window_point);
                Message::ImageMouseMessage(MouseMessage::Over(image_point))
            })
            .on_press(Message::ImageMouseMessage(MouseMessage::Press))
            .on_release(Message::ImageMouseMessage(MouseMessage::Release));
        image_mouse_area.into()
    }

    fn view_debugger(&self) -> iced::Element<Message> {
        let debug_str: String = format!("{:?}, {:?}, {:?}, {:?}", self.mouse_position, self.mouse_state, self.view_mode, self.workspace.current_crop());
        iced::widget::container(iced::widget::text(debug_str))
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
                iced::widget::button("Crop").on_press(Message::ToggleCropMode),
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