use crate::{pipeline::viewport::Viewport, ui::message::TaskMessage, update_event::{AlbumEvent, MouseEvent, UpdateEvent, WorkspaceEvent}, workspace::{image_loader, workspace::Workspace}, Main, Message, MouseState, ViewMode};

use std::{path::PathBuf, usize};

impl Main {
    pub fn update(&mut self, message: Message) -> iced::Task<Message> {
        let update_event = UpdateEvent::from(message);
        match update_event {
            UpdateEvent::OnStart => {
                self.batch_image_load()
            },
            UpdateEvent::AlbumEvent(album_event) => {
                self.update_album(album_event)
            },
            UpdateEvent::WorkspaceEvent(workspace_event) => {
                self.update_workspace(workspace_event);
                iced::Task::none()
            }
        }
    }

    fn update_album(&mut self, album_event: AlbumEvent) -> iced::Task<Message> {
        self.album.update_workspace(&self.workspace);
        let tasks = match album_event {
            AlbumEvent::LoadAlbum => {
                self.open_file_dialog();
                self.batch_image_load()
            },
            AlbumEvent::SaveAlbum => {
                self.album.save();
                iced::Task::none()
            },
            AlbumEvent::NextImage => {
                self.album.next_image();
                iced::Task::none()
            },
            AlbumEvent::SetImage(index) => {
                self.album.set_image_index(index);
                iced::Task::none()
            }
            AlbumEvent::DeleteImage => {
                self.album.delete_image();
                iced::Task::none()
            },
            AlbumEvent::LoadImage(photo_id, image) => {
                self.album.set_image(photo_id, image);
                iced::Task::none()
            }
        };
        self.workspace = self.album.make_workspace();
        self.viewport = self.workspace.as_ref().and_then(Viewport::try_new);
        tasks
    }

    fn update_workspace(&mut self, workspace_event: WorkspaceEvent) {
        if let Some(workspace) = &mut self.workspace {
            match workspace_event {
                WorkspaceEvent::ToggleCropMode => {
                    workspace.toggle_view_mode(ViewMode::Crop);
                },
                WorkspaceEvent::ToggleMaskMode(mask_index) => {
                    workspace.toggle_view_mode(ViewMode::Mask(mask_index));
                },
                WorkspaceEvent::BrightnessChanged(brightness) => {
                    workspace.set_brightness(brightness);
                },
                WorkspaceEvent::ContrastChanged(contrast) => {
                    workspace.set_contrast(contrast);
                },
                WorkspaceEvent::TintChanged(tint) => {
                    workspace.set_tint(tint);
                },
                WorkspaceEvent::TemperatureChanged(temperature) => {
                    workspace.set_temperature(temperature);
                },
                WorkspaceEvent::SaturationChanged(saturation) => {
                    workspace.set_saturation(saturation);
                },
                WorkspaceEvent::AddMask => {
                    workspace.add_mask();
                },
                WorkspaceEvent::DeleteMask(index) => {
                    workspace.delete_mask(index);
                },
                WorkspaceEvent::MaskToggleLinear(index, is_linear) => {
                    workspace.set_mask_is_linear(index, is_linear);
                },
                WorkspaceEvent::MaskBrightnessChanged(index, brightness) => {
                    workspace.set_mask_brightness(index, brightness);
                },
                WorkspaceEvent::MaskAngleChanged(index, angle) => {
                    workspace.set_mask_angle(index, angle);
                },
                WorkspaceEvent::AngleChanged(angle_degrees) => {
                    workspace.set_crop_angle(angle_degrees);
                },
                WorkspaceEvent::ExportImage => {
                    workspace.export_image();
                },
                WorkspaceEvent::ImageMouseEvent(mouse_event) => {
                    Self::update_mouse_on_image(workspace, mouse_event);
                }
            }

            self.viewport = Viewport::try_new(workspace);
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

    fn update_mouse_on_image(workspace: &mut Workspace, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Press(_) => workspace.set_mouse_state(MouseState::Down),
            MouseEvent::Release => workspace.set_mouse_state(MouseState::Up),
            _ => {}
        }

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
            MouseEvent::Over(mouse_position) => {
                if matches!(workspace.get_mouse_state(), MouseState::Down) {
                    workspace.update_view_offset(mouse_position.relative_x, mouse_position.relative_y);
                }
            },
            MouseEvent::Press(mouse_position) => {
                workspace.new_view_offset_origin(mouse_position.relative_x, mouse_position.relative_y);
            },
            _ => {}
        }
    }

    fn update_mouse_crop_mode(workspace: &mut Workspace, mouse_event: MouseEvent) {
        match mouse_event {
            MouseEvent::Over(mouse_position) => {
                if matches!(workspace.get_mouse_state(), MouseState::Down) {
                    workspace.update_crop(mouse_position.image_x, mouse_position.image_y);
                }
            },
            MouseEvent::Press(mouse_position) => {
                workspace.new_crop(mouse_position.image_x, mouse_position.image_y);
            },
            _ => {}
        }
    }

    fn update_mouse_mask_mode(workspace: &mut Workspace, mouse_event: MouseEvent, mask_index: usize) {
        match mouse_event {
            MouseEvent::Over(mouse_position) => {
                if matches!(workspace.get_mouse_state(), MouseState::Down) {
                    workspace.update_mask_radius(mask_index, mouse_position.image_x, mouse_position.image_y);
                }
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

    fn batch_image_load(&self) -> iced::Task<Message> {
        iced::Task::batch(self.album.iter_images()
            .map(|image| {
                let photo_id = image.photo_id;
                let path = image.path.clone();
                iced::Task::perform(
                    image_loader::load_image(photo_id, path),
                    TaskMessage::NewImage)
            }))
            .map(Message::TaskMessage)
    }
}