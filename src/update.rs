use crate::{pipeline::viewport::{self, Viewport}, Main, Message, MouseMessage, MouseState, Point, ViewMode};

use std::usize;

#[derive(Clone, Copy)]
enum MouseEvent {
    Press(i32, i32),
    Release(i32, i32),
    RightPress(i32, i32),
    Down(i32, i32),
    Over(i32, i32),
    Scroll(f32)
}

impl Main {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        match message {
            Message::LoadAlbum => {
                // self.open_file_dialog()
            },
            Message::NextImage => {
                self.workspace.next_image_index();
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
            Message::DeleteMask(mask_index) => {
                self.workspace.delete_mask(mask_index);
            },
            Message::MaskBrightnessChanged(index, brightness) => {
                self.workspace.set_mask_brightness(index, brightness);
            },
            Message::AngleChanged(angle_degrees) => {
                self.workspace.set_crop_angle(angle_degrees);
            },
            Message::ImageMouseMessage(image_mouse_message) => {
                self.update_mouse_on_image(image_mouse_message);
            }
        };

        if self.workspace.get_has_updated() {
            self.viewport = Viewport::from_workspace(&self.workspace);
            self.workspace.reset_has_updated();
        }

        iced::Task::none()
    }

    // TODO: Figure out what to do with this
    // fn open_file_dialog(&mut self) -> bool {
    //     let path: PathBuf = std::env::current_dir().unwrap();

    //     let result = native_dialog::FileDialog::new()
    //         .set_location(&path)
    //         .add_filter("image", &["png", "jpg"])
    //         .show_open_multiple_file();

    //     match result {
    //         Ok(file_paths) => {
    //             self.workspace = workspace::load_workspace(&file_paths);
    //             true
    //         },
    //         _ => {
    //             false
    //         }
    //     }
    // }

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
            MouseEvent::RightPress(x, y) => {
                self.workspace.white_balance_at(x, y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                self.workspace.update_zoom(scroll_delta);
            },
            _ => {}
        }
    }

    fn update_mouse_crop_mode(&mut self, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Down(x, y) => {
                self.workspace.update_crop(x, y);
            },
            MouseEvent::Press(x, y) => {
                self.workspace.new_crop(x, y);
            },
            _ => {}
        }
    }

    fn update_mouse_mask_mode(&mut self, mouse_event: MouseEvent, mask_index: usize) {
        match mouse_event {
            MouseEvent::Down(x, y) => {
                self.workspace.update_mask_radius(mask_index, x, y);
            },
            MouseEvent::Press(x, y) => {
                self.workspace.update_mask_position(mask_index, x, y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                self.workspace.update_zoom(scroll_delta);
            },
            _ => {}
        }
    }

    fn to_mouse_event(&mut self, image_mouse_message: MouseMessage) -> MouseEvent {
        let x: i32 = viewport::get_image_mouse_x();
        let y: i32 = viewport::get_image_mouse_y();
        match image_mouse_message {
            MouseMessage::Over => {
                self.mouse_position = Point { x, y };
                match self.mouse_state {
                    MouseState::Down => MouseEvent::Down(x, y),
                    MouseState::Up => MouseEvent::Over(x, y),
                }
            },
            MouseMessage::Press => {
                self.mouse_state = MouseState::Down;
                MouseEvent::Press(x, y)
            },
            MouseMessage::Release => {
                self.mouse_state = MouseState::Up;
                MouseEvent::Release(x, y)
            },
            MouseMessage::RightPress => {
                MouseEvent::RightPress(x, y)
            },
            MouseMessage::Scroll(scroll_delta) => {
                MouseEvent::Scroll(scroll_delta)
            }
        }
    }
}