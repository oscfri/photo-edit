use crate::{pipeline::viewport, workspace, Main, Message, MouseMessage, MouseState, Point, ViewMode};

use std::usize;

// TODO: This function should move to somewhere else...
pub fn make_viewport(workspace: &workspace::Workspace) -> viewport::Viewport {
    viewport::Viewport::new(workspace.make_viewport(), workspace.get_view_mode())
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
            self.update_image_task();
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
            MouseMessage::Release => {
                self.mouse_state = MouseState::Up;
            },
            _ => {}
        }
        
        match self.workspace.get_view_mode() {
            ViewMode::Normal => {
                self.update_mouse_normal_mode(image_mouse_message);
            },
            ViewMode::Crop => {
                self.update_mouse_crop_mode(image_mouse_message);
            },
            ViewMode::Mask(mask_index) => {
                self.update_mouse_mask_mode(image_mouse_message, mask_index);
            }
        }
    }

    fn update_mouse_normal_mode(&mut self, image_mouse_message: MouseMessage) {
        match image_mouse_message {
            MouseMessage::RightPress => {
                let x: usize = viewport::get_image_mouse_x() as usize;
                let y: usize = viewport::get_image_mouse_y() as usize;
                self.workspace.white_balance_at(x, y);
            },
            MouseMessage::Scroll(scroll_delta) => {
                self.workspace.update_zoom(scroll_delta);
            },
            _ => {}
        }
    }

    fn update_mouse_crop_mode(&mut self, image_mouse_message: MouseMessage) {
        match image_mouse_message {
            MouseMessage::Over => {
                match self.mouse_state {
                    MouseState::Down => {
                        let x: i32 = viewport::get_image_mouse_x();
                        let y: i32 = viewport::get_image_mouse_y();
                        self.workspace.update_crop(x, y);
                    },
                    _ => {}
                }
            },
            MouseMessage::Press => {
                let x: i32 = viewport::get_image_mouse_x();
                let y: i32 = viewport::get_image_mouse_y();
                self.workspace.new_crop(x, y);
            },
            _ => {}
        }
    }

    fn update_mouse_mask_mode(&mut self, image_mouse_message: MouseMessage, mask_index: usize) {
        match image_mouse_message {
            MouseMessage::Over => {
                match self.mouse_state {
                    MouseState::Down => {
                        let x: i32 = viewport::get_image_mouse_x();
                        let y: i32 = viewport::get_image_mouse_y();
                        self.workspace.update_mask_radius(mask_index, x, y);
                    },
                    _ => {}
                }
            },
            MouseMessage::Press => {
                let x: i32 = viewport::get_image_mouse_x();
                let y: i32 = viewport::get_image_mouse_y();
                self.workspace.update_mask_position(mask_index, x, y);
            },
            MouseMessage::Scroll(scroll_delta) => {
                self.workspace.update_zoom(scroll_delta);
            },
            _ => {}
        }
    }
    
    fn update_image_task(&mut self) {
        self.viewport = make_viewport(&self.workspace);
    }
}