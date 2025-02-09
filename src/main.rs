mod album;
mod types;
mod pipeline;
mod workspace;
mod view_mode;
mod view;

use album::{AlbumImage, Crop};
use iced::{self, widget::container};
use native_dialog;
use pipeline::viewport;
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
    Over,
    Press,
    RightPress,
    Release
}

#[derive(Debug, Clone)]
enum Message {
    LoadAlbum,
    NextImage,
    SetImage(usize),
    ToggleCropMode,
    ToggleMaskMode(usize),
    BrightnessChanged(f32),
    ContrastChanged(f32),
    TintChanged(f32),
    TemperatureChanged(f32),
    SaturationChanged(f32),
    MaskBrightnessChanged(usize, f32),
    AngleChanged(f32),
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
    viewport::Viewport::new(
            workspace.make_viewport(&view_mode),
            view_mode.clone())
}

fn calculate_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f32).sqrt()
}

impl<'a> Main {

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
            Message::ToggleMaskMode(mask_index) => {
                if !matches!(self.view_mode, ViewMode::Mask(mask_index)) {
                    self.view_mode = ViewMode::Mask(mask_index);
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
            Message::MaskBrightnessChanged(index, brightness) => {
                self.workspace.current_parameters_mut().radial_masks[index].brightness = brightness;
                true
            },
            Message::AngleChanged(angle) => {
                self.workspace.current_crop_mut().angle_degrees = angle;
                true
            },
            Message::ImageMouseMessage(image_mouse_message) => {
                self.update_mouse_on_image(image_mouse_message)
            }
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
            MouseMessage::Over => {
                self.mouse_position = Point {
                    x: viewport::get_image_mouse_x(),
                    y: viewport::get_image_mouse_y(),
                };
            },
            MouseMessage::Press => {
                self.mouse_state = MouseState::Down;
            },
            MouseMessage::RightPress => {}, // Do nothing
            MouseMessage::Release => {
                self.mouse_state = MouseState::Up;
            },
        }
        
        match self.view_mode {
            ViewMode::Normal => {
                self.update_mouse_normal_mode(image_mouse_message)
            },
            ViewMode::Crop => {
                self.update_mouse_crop_mode(image_mouse_message)
            },
            ViewMode::Mask(mask_index) => {
                self.update_mouse_mask_mode(image_mouse_message, mask_index)
            }
        }
    }

    fn update_mouse_normal_mode(&mut self, image_mouse_message: MouseMessage) -> bool {
        match image_mouse_message {
            MouseMessage::Over => {
                false
            },
            MouseMessage::Press => {
                false
            },
            MouseMessage::RightPress => {
                // White balance
                let x: usize = viewport::get_image_mouse_x() as usize;
                let y: usize = viewport::get_image_mouse_y() as usize;
                let current_image: &album::AlbumImage = self.workspace.current_image();
                match current_image.lab_pixel_at(x, y) {
                    Some(pixel) => {
                        let parameters: &mut album::Parameters = self.workspace.current_parameters_mut();
                        parameters.tint = -pixel.tint;
                        parameters.temperature = -pixel.temperature;
                        true
                    },
                    None => {
                        false
                    }
                }
            },
            MouseMessage::Release => {
                false
            }
        }
    }

    fn update_mouse_crop_mode(&mut self, image_mouse_message: MouseMessage) -> bool {
        match image_mouse_message {
            MouseMessage::Over => {
                match self.mouse_state {
                    MouseState::Up => {
                        false
                    },
                    MouseState::Down => {
                        let crop: &mut Crop = self.workspace.current_crop_mut();
                        let width: f32 = (viewport::get_image_mouse_x() - crop.center_x) as f32;
                        let height: f32 = (viewport::get_image_mouse_y() - crop.center_y) as f32;
                        let angle: f32 = crop.angle_degrees / 180.0 * std::f32::consts::PI;
                        let sin: f32 = f32::sin(angle);
                        let cos: f32 = f32::cos(angle);
                        crop.width = ((width * cos + height * sin).abs() * 2.0) as i32;
                        crop.height = ((-width * sin + height * cos).abs() * 2.0) as i32;
                        true
                    }
                }
            },
            MouseMessage::Press => {
                let crop: &mut Crop = self.workspace.current_crop_mut();
                crop.center_x = viewport::get_image_mouse_x();
                crop.center_y = viewport::get_image_mouse_y();
                crop.width = 0;
                crop.height = 0;
                true
            },
            MouseMessage::RightPress | MouseMessage::Release => {
                false
            }
        }
    }

    fn update_mouse_mask_mode(&mut self, image_mouse_message: MouseMessage, mask_index: usize) -> bool {
        match image_mouse_message {
            MouseMessage::Over => {
                match self.mouse_state {
                    MouseState::Up => {
                        false
                    },
                    MouseState::Down => {
                        let parameters: &mut album::Parameters = self.workspace.current_parameters_mut();
                        let center_x = parameters.radial_masks[mask_index].center_x;
                        let center_y = parameters.radial_masks[mask_index].center_y;
                        parameters.radial_masks[mask_index].radius = calculate_distance(
                            center_x,
                            center_y,
                            viewport::get_image_mouse_x(),
                            viewport::get_image_mouse_y());
                        true
                    }
                }
            },
            MouseMessage::Press => {
                let parameters: &mut album::Parameters = self.workspace.current_parameters_mut();
                parameters.radial_masks[mask_index].center_x = viewport::get_image_mouse_x();
                parameters.radial_masks[mask_index].center_y = viewport::get_image_mouse_y();
                parameters.radial_masks[mask_index].radius = 0.0;
                true
            },
            MouseMessage::RightPress => {
                false
            },
            MouseMessage::Release => {
                false
            }
        }
    }
    
    fn update_image_task(&mut self) {
        self.viewport = make_viewport(&self.workspace, &self.view_mode);
    }
    
    fn view(&self) -> iced::Element<Message> {
        let view = view::View::new(
            &self.viewport,
            self.mouse_position,
            self.workspace.current_crop(),
            self.workspace.current_parameters(),
            self.workspace.album_images());
        view.view()
    }
}

impl Default for Main {
    fn default() -> Self {
        Self::new()
    }
}