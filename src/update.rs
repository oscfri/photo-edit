use crate::{pipeline::viewport::{self, Viewport}, ui::message::{AlbumMessage, WorkspaceMessage}, workspace::workspace::Workspace, Main, Message, MouseMessage, MouseState, ViewMode};

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
            Message::AlbumMessage(album_message) => {
                self.update_album(album_message);
            },
            Message::WorkspaceMessage(workspace_message) => {
                self.update_workspace(workspace_message);
            }
        };

        iced::Task::none()
    }

    fn update_album(&mut self, album_message: AlbumMessage) {
        self.album.update_workspace(&self.workspace);
        match album_message {
            AlbumMessage::LoadAlbum => {
                self.open_file_dialog()
            },
            AlbumMessage::SaveAlbum => {
                self.album.save();
            },
            AlbumMessage::NextImage => {
                self.album.next_image();
            },
            AlbumMessage::SetImage(index) => {
                self.album.set_image(index);
            }
            AlbumMessage::DeleteImage => {
                self.album.delete_image();
            },
        }
        self.workspace = self.album.make_workspace();
        self.viewport = self.workspace.as_ref().map(Viewport::new);
    }

    fn update_workspace(&mut self, workspace_message: WorkspaceMessage) {
        if let Some(workspace) = &mut self.workspace {
            match workspace_message {
                WorkspaceMessage::ToggleCropMode => {
                    workspace.toggle_view_mode(ViewMode::Crop);
                },
                WorkspaceMessage::ToggleMaskMode(mask_index) => {
                    workspace.toggle_view_mode(ViewMode::Mask(mask_index));
                },
                WorkspaceMessage::BrightnessChanged(brightness) => {
                    workspace.set_brightness(brightness);
                },
                WorkspaceMessage::ContrastChanged(contrast) => {
                    workspace.set_contrast(contrast);
                },
                WorkspaceMessage::TintChanged(tint) => {
                    workspace.set_tint(tint);
                },
                WorkspaceMessage::TemperatureChanged(temperature) => {
                    workspace.set_temperature(temperature);
                },
                WorkspaceMessage::SaturationChanged(saturation) => {
                    workspace.set_saturation(saturation);
                },
                WorkspaceMessage::AddMask => {
                    workspace.add_mask();
                },
                WorkspaceMessage::DeleteMask(index) => {
                    workspace.delete_mask(index);
                },
                WorkspaceMessage::MaskToggleLinear(index, is_linear) => {
                    workspace.set_mask_is_linear(index, is_linear);
                },
                WorkspaceMessage::MaskBrightnessChanged(index, brightness) => {
                    workspace.set_mask_brightness(index, brightness);
                },
                WorkspaceMessage::MaskAngleChanged(index, angle) => {
                    workspace.set_mask_angle(index, angle);
                },
                WorkspaceMessage::AngleChanged(angle_degrees) => {
                    workspace.set_crop_angle(angle_degrees);
                },
                WorkspaceMessage::ExportImage => {
                    workspace.export_image();
                },
                WorkspaceMessage::ImageMouseMessage(image_mouse_message) => {
                    Self::update_mouse_on_image(workspace, image_mouse_message);
                }
            }

            self.viewport = Some(Viewport::new(workspace));
        }
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

            self.album = self.album_factory.create()
        }
    }

    fn update_mouse_on_image(workspace: &mut Workspace, image_mouse_message: MouseMessage) {
        let mouse_event: MouseEvent = Self::to_mouse_event(workspace, image_mouse_message);
        
        match workspace.get_view_mode() {
            ViewMode::Normal => {
                Self::update_mouse_normal_mode(workspace, mouse_event);
            },
            ViewMode::Crop => {
                Self::update_mouse_crop_mode(workspace, mouse_event);
            },
            ViewMode::Mask(mask_index) => {
                Self::update_mouse_mask_mode(workspace, mouse_event, mask_index);
            }
        }
    }

    fn update_mouse_normal_mode(workspace: &mut Workspace, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::RightPress(mouse_position) => {
                workspace.white_balance_at(mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                workspace.update_view_zoom(scroll_delta);
            },
            MouseEvent::Down(mouse_position) => {
                workspace.update_view_offset(mouse_position.relative_x, mouse_position.relative_y);
            },
            MouseEvent::Press(mouse_position) => {
                workspace.new_view_offset_origin(mouse_position.relative_x, mouse_position.relative_y);
            },
            _ => {}
        }
    }

    fn update_mouse_crop_mode(workspace: &mut Workspace, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Down(mouse_position) => {
                workspace.update_crop(mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Press(mouse_position) => {
                workspace.new_crop(mouse_position.image_x, mouse_position.image_y);
            },
            _ => {}
        }
    }

    fn update_mouse_mask_mode(workspace: &mut Workspace, mouse_event: MouseEvent, mask_index: usize) {
        match mouse_event {
            MouseEvent::Down(mouse_position) => {
                workspace.update_mask_radius(mask_index, mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Press(mouse_position) => {
                workspace.update_mask_position(mask_index, mouse_position.image_x, mouse_position.image_y);
            },
            MouseEvent::Scroll(scroll_delta) => {
                workspace.update_view_zoom(scroll_delta);
            },
            _ => {}
        }
    }

    fn to_mouse_event(workspace: &mut Workspace, image_mouse_message: MouseMessage) -> MouseEvent {
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
        // TODO: Move this to workspace
        match image_mouse_message {
            MouseMessage::Over => {
                match workspace.get_mouse_state() {
                    MouseState::Down => MouseEvent::Down(mouse_position),
                    MouseState::Up => MouseEvent::Over(mouse_position),
                }
            },
            MouseMessage::Press => {
                workspace.set_mouse_state(MouseState::Down);
                MouseEvent::Press(mouse_position)
            },
            MouseMessage::Release => {
                workspace.set_mouse_state(MouseState::Up);
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