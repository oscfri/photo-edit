use crate::{pipeline::viewport::{self, Viewport}, Main, Message, MouseMessage, MouseState, Point, ViewMode};

use std::{path::PathBuf, usize};

#[derive(Clone, Copy)]
struct MousePosition {
    image_x: i32,
    image_y: i32,
    relative_x: i32,
    relative_y: i32
}

#[derive(Clone, Copy)]
enum MouseEvent {
    Press(MousePosition),
    Release(MousePosition),
    RightPress(MousePosition),
    Down(MousePosition),
    Over(MousePosition),
    Scroll(f32)
}

impl Main {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::LoadAlbum => {
                self.open_file_dialog()
            },
            Message::SaveAlbum => {
                self.workspace.save_album(&self.repository);
            },
            Message::ExportImage => {
                self.workspace.export_image();
            },
            Message::NextImage => {
                self.workspace.next_image_index();
            },
            Message::DeleteImage => {
                self.delete_current_image();
            },
            Message::SetImage(index) => {
                self.workspace.set_image_index(index);
            },
            Message::ToggleCropMode => {
                self.workspace.toggle_view_mode(ViewMode::Crop);
            },
            Message::ToggleMaskMode(mask_index) => {
                self.workspace.toggle_view_mode(ViewMode::Mask(mask_index));
            },
            Message::BrightnessChanged(brightness) => {
                self.workspace.set_brightness(brightness);
            },
            Message::ContrastChanged(contrast) => {
                self.workspace.set_contrast(contrast);
            },
            Message::TintChanged(tint) => {
                self.workspace.set_tint(tint);
            },
            Message::TemperatureChanged(temperature) => {
                self.workspace.set_temperature(temperature);
            },
            Message::SaturationChanged(saturation) => {
                self.workspace.set_saturation(saturation);
            },
            Message::AddMask => {
                self.workspace.add_mask();
            },
            Message::DeleteMask(index) => {
                self.workspace.delete_mask(index);
            },
            Message::MaskToggleLinear(index, is_linear) => {
                self.workspace.set_mask_is_linear(index, is_linear);
            },
            Message::MaskBrightnessChanged(index, brightness) => {
                self.workspace.set_mask_brightness(index, brightness);
            },
            Message::MaskAngleChanged(index, angle) => {
                self.workspace.set_mask_angle(index, angle);
            },
            Message::AngleChanged(angle_degrees) => {
                self.workspace.set_crop_angle(angle_degrees);
            },
            Message::ImageMouseMessage(image_mouse_message) => {
                self.update_mouse_on_image(image_mouse_message);
            }
        };

        if self.workspace.get_has_updated() {
            self.viewport = Viewport::try_from(&self.workspace);
            self.workspace.reset_has_updated();
        }

        iced::Task::none()
    }

    fn open_file_dialog(&mut self) {
        let path: PathBuf = std::env::current_dir().unwrap();

        let result = native_dialog::FileDialog::new()
            .set_location(&path)
            .add_filter("image", &["png", "jpg"])
            .show_open_multiple_file();

        if let Ok(file_paths) = result {
            for file_path in file_paths {
                self.repository.add_photo(&file_path).ok();
            }
            // TODO: A bit excessive to reload entire album
            self.workspace = self.workspace_factory.create();
        }
    }

    fn delete_current_image(&mut self) {
        let photo_id = self.workspace.current_image().photo_id;
        self.repository.delete_photo(photo_id).ok();
        // TODO: A bit excessive to reload entire album
        self.workspace = self.workspace_factory.create();
    }

    fn update_mouse_on_image(&mut self, image_mouse_message: MouseMessage) {
        let mouse_event: MouseEvent = self.to_mouse_event(image_mouse_message);
        
        match self.workspace.get_view_mode() {
            ViewMode::Normal => {
                self.update_mouse_normal_mode(mouse_event);
            },
            ViewMode::Crop => {
                self.update_mouse_crop_mode(mouse_event);
            },
            ViewMode::Mask(mask_index) => {
                self.update_mouse_mask_mode(mouse_event, mask_index);
            }
        }
    }

    fn update_mouse_normal_mode(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::RightPress(mouse_position) => {
                self.workspace.white_balance_at(mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                self.workspace.update_view_zoom(scroll_delta);
            },
            MouseEvent::Down(mouse_position) => {
                self.workspace.update_view_offset(mouse_position.relative_x, mouse_position.relative_y);
            },
            MouseEvent::Press(mouse_position) => {
                self.workspace.new_view_offset_origin(mouse_position.relative_x, mouse_position.relative_y);
            },
            _ => {}
        }
    }

    fn update_mouse_crop_mode(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Down(mouse_position) => {
                self.workspace.update_crop(mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Press(mouse_position) => {
                self.workspace.new_crop(mouse_position.image_x, mouse_position.image_y);
            },
            _ => {}
        }
    }

    fn update_mouse_mask_mode(&mut self, mouse_event: MouseEvent, mask_index: usize) {
        match mouse_event {
            MouseEvent::Down(mouse_position) => {
                self.workspace.update_mask_radius(mask_index, mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Press(mouse_position) => {
                self.workspace.update_mask_position(mask_index, mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                self.workspace.update_view_zoom(scroll_delta);
            },
            _ => {}
        }
    }

    fn to_mouse_event(&mut self, image_mouse_message: MouseMessage) -> MouseEvent {
        let image_mouse_x: i32 = viewport::get_image_mouse_x();
        let image_mouse_y: i32 = viewport::get_image_mouse_y();
        let relative_mouse_x: i32 = viewport::get_relative_mouse_x();
        let relative_mouse_y: i32 = viewport::get_relative_mouse_y();
        let mouse_position: MousePosition = MousePosition {
            image_x: image_mouse_x, 
            image_y: image_mouse_y,
            relative_x: relative_mouse_x,
            relative_y: relative_mouse_y
        };
        match image_mouse_message {
            MouseMessage::Over => {
                self.mouse_position = Point { x: mouse_position.image_x, y: mouse_position.image_y };
                match self.mouse_state {
                    MouseState::Down => MouseEvent::Down(mouse_position),
                    MouseState::Up => MouseEvent::Over(mouse_position),
                }
            },
            MouseMessage::Press => {
                self.mouse_state = MouseState::Down;
                MouseEvent::Press(mouse_position)
            },
            MouseMessage::Release => {
                self.mouse_state = MouseState::Up;
                MouseEvent::Release(mouse_position)
            },
            MouseMessage::RightPress => {
                MouseEvent::RightPress(mouse_position)
            },
            MouseMessage::Scroll(scroll_delta) => {
                MouseEvent::Scroll(scroll_delta)
            }
        }
    }
}