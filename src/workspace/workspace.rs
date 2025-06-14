use core::f32;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use crate::pipeline::export_image::export_image;
use crate::pipeline::viewport::{ViewportCrop, ViewportParameters};
use crate::types::{LabPixel, RawImage};
use crate::ui::message::MouseState;
use crate::view_mode::ViewMode;
use crate::view_mode;

use super::parameters::{CropPreset, Parameter, ParameterHistory, Parameters, RadialMask};

#[derive(Clone)]
pub struct WorkspaceImage {
    photo_id: i32,
    image: Option<Arc<RawImage>>,
    parameter_history: Arc<Mutex<ParameterHistory>>,
    image_view: Arc<Mutex<ImageView>>,
    file_name: String
}

impl WorkspaceImage {
    pub fn new(
            photo_id: i32,
            image: Option<Arc<RawImage>>,
            parameter_history: Arc<Mutex<ParameterHistory>>,
            image_view: Arc<Mutex<ImageView>>,
            file_name: String) -> Self {
        Self {
            photo_id,
            image,
            parameter_history,
            image_view,
            file_name
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ImageView {
    pub offset_x: f32,
    pub offset_y: f32,
    pub zoom: f32,
}

impl ImageView {
    pub fn get_offset_x(&self) -> f32 {
        self.offset_x
    }

    pub fn get_offset_y(&self) -> f32 {
        self.offset_y
    }

    pub fn update_offset(&mut self, x: f32, y: f32) {
        self.offset_x = x;
        self.offset_y = y;
    }

    pub fn get_zoom(&self) -> f32 {
        self.zoom
    }

    pub fn update_zoom(&mut self, zoom_delta: f32) {
        self.zoom += zoom_delta;

        if self.zoom < 0.0 {
            self.zoom = 0.0;
        } else if self.zoom > 4.0 {
            self.zoom = 4.0
        } else if self.zoom.abs() < 1e-5 {
            // Snap to 0 to make default comparison more reliable
            self.zoom = 0.0
        }
    }

    pub fn reset(&mut self) {
        *self = Self::default()
    }
}

#[derive(Clone)]
pub struct Workspace {
    image: WorkspaceImage,
    view_mode: ViewMode,
    parameters_visible: bool,

    // For view/crop dragging (there's probably a better way to handle this)
    mouse_state: MouseState,
    mouse_origin_x: i32,
    mouse_origin_y: i32,
    offset_origin_x: i32,
    offset_origin_y: i32,
}

impl Workspace {
    pub fn new(image: WorkspaceImage) -> Self {
        Self {
            image,
            view_mode: ViewMode::Normal,
            parameters_visible: true,
            mouse_state: MouseState::Up,
            mouse_origin_x: 0,
            mouse_origin_y: 0,
            offset_origin_x: 0,
            offset_origin_y: 0,
        }
    }

    pub fn update(image: WorkspaceImage, workspace: &Option<Self>) -> Self {
        if let Some(workspace) = workspace {
            Self {
                image,
                ..workspace.clone()
            }
        } else {
            Self::new(image)
        }
    }

    pub fn get_photo_id(&self) -> i32 {
        self.image.photo_id
    }

    pub fn get_file_name(&self) -> String {
        self.image.file_name.clone()
    }

    pub fn get_mouse_state(&self) -> MouseState {
        self.mouse_state
    }

    pub fn set_mouse_state(&mut self, mouse_state: MouseState) {
        self.mouse_state = mouse_state
    }

    pub fn current_source_image(&self) -> Option<Arc<RawImage>> {
        self.image.image.clone()
    }

    pub fn current_parameters(&self) -> Parameters {
        self.image.parameter_history.lock().unwrap().current()
    }

    pub fn get_parameters_visible(&self) -> bool {
        self.parameters_visible
    }

    pub fn parameters_to_display(&self) -> ViewportParameters {
        let parameters = self.image.parameter_history.lock().unwrap().current().into();

        if self.parameters_visible {
            parameters
        } else {
            ViewportParameters {
                crop: parameters.crop,
                ..ViewportParameters::default()
            }
        }
    }

    pub fn can_reset_view(&self) -> bool {
        !self.image.image_view.lock().unwrap().eq(&ImageView::default())
    }

    pub fn current_image_view(&self) -> ImageView {
        self.image.image_view.lock().unwrap().clone()
    }

    pub fn current_angle_degrees(&self) -> f32 {
        self.current_parameters().crop.as_ref()
            .map_or(0.0, |crop| crop.angle_degrees)
    }

    pub fn current_crop_scale(&self) -> f32 {
        self.current_parameters().crop.as_ref()
            .map_or(0.0, |crop| crop.scale)
    }

    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode
    }

    pub fn is_crop_mode(&self) -> bool {
        matches!(self.view_mode, ViewMode::Crop)
    }

    pub fn get_mask_index(&self) -> Option<usize> {
        if let ViewMode::Mask(index) = self.view_mode {
            Some(index)
        } else {
            None
        }
    }

    pub fn export_image(&self, export_directory: PathBuf) {
        futures_executor::block_on(export_image(&self, export_directory));
    }

    pub fn undo(&mut self) {
        self.image.parameter_history.lock().unwrap().undo()
    }

    pub fn redo(&mut self) {
        self.image.parameter_history.lock().unwrap().redo()
    }

    pub fn copy_parameters(&mut self) -> Parameters {
        self.current_parameters()
    }

    pub fn paste_parameters(&mut self, clipboard_parameters: &Parameters) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.base_parameters = clipboard_parameters.base_parameters.clone());
    }

    pub fn decrease_last_parameter(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update_last_f32(|value| *value = *value - 0.1);
    }

    pub fn decrease_last_parameter_large(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update_last_f32(|value| *value = *value - 1.0);
    }

    pub fn increase_last_parameter(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update_last_f32(|value| *value = *value + 0.1);
    }

    pub fn increase_last_parameter_large(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update_last_f32(|value| *value = *value + 1.0);
    }

    pub fn toggle_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = self.view_mode.toggle_view_mode(view_mode);
    }

    pub fn set_exposure(&mut self, exposure: f32) {
        self.set_parameter_value(Parameter::Exposure, exposure);
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.set_parameter_value(Parameter::Contrast, contrast);
    }

    pub fn set_shadows(&mut self, shadows: f32) {
        self.set_parameter_value(Parameter::Shadows, shadows);
    }

    pub fn set_midtones(&mut self, midtones: f32) {
        self.set_parameter_value(Parameter::Midtones, midtones);
    }

    pub fn set_highlights(&mut self, highlights: f32) {
        self.set_parameter_value(Parameter::Highlights, highlights);
    }

    pub fn set_tint(&mut self, tint: f32) {
        self.set_parameter_value(Parameter::Tint, tint);
    }
    
    pub fn set_temperature(&mut self, temperature: f32) {
        self.set_parameter_value(Parameter::Temperature, temperature);
    }
    
    pub fn set_saturation(&mut self, saturation: f32) {
        self.set_parameter_value(Parameter::Saturation, saturation);
    }

    pub fn add_mask(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let new_mask_index = parameters.radial_masks.len();
                parameters.radial_masks.push(RadialMask::default());
                self.view_mode = ViewMode::Mask(new_mask_index);
            });
    }

    pub fn delete_mask(&mut self, mask_index: usize) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                parameters.radial_masks.remove(mask_index);
            });
        self.view_mode = ViewMode::Normal;
    }

    pub fn update_mask_position(&mut self, mask_index: usize, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                radial_mask.center_x = x;
                radial_mask.center_y = y;
                radial_mask.width = 0;
                radial_mask.height = 0;
            });
    }

    pub fn update_mask_size(&mut self, mask_index: usize, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                let center_x = radial_mask.center_x;
                let center_y = radial_mask.center_y;
                let delta_x = (center_x - x) as f32;
                let delta_y = (center_y - y) as f32;
                let angle = radial_mask.angle_degrees / 180.0 * std::f32::consts::PI;
                radial_mask.width = (angle.cos() * delta_x - angle.sin() * delta_y).abs() as i32;
                radial_mask.height = (angle.sin() * delta_x + angle.cos() * delta_y).abs() as i32;
            });
    }

    pub fn set_mask_is_linear(&mut self, mask_index: usize, is_linear: bool) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.radial_masks[mask_index].is_linear = is_linear);
    }

    pub fn set_mask_brightness(&mut self, mask_index: usize, brightness: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.radial_masks[mask_index].brightness = brightness);
    }

    pub fn set_mask_angle_degrees(&mut self, mask_index: usize, angle_degrees: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                radial_mask.angle_degrees = angle_degrees;
            });
    }

    pub fn set_mask_feather(&mut self, mask_index: usize, feather: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                radial_mask.feather = feather;
            });
    }

    pub fn set_crop_angle(&mut self, angle_degrees: f32) {
        self.set_parameter_value(Parameter::CropAngle, angle_degrees);
    }

    pub fn set_crop_scale(&mut self, scale: f32) {
        self.set_parameter_value(Parameter::CropScale, scale);
    }

    pub fn update_crop_scale(&mut self, scroll_delta: f32) {
        let current_scale = self.current_crop_scale();
        let new_scale = (current_scale - scroll_delta * 0.05).clamp(-5.0, 0.0);

        self.set_parameter_value(Parameter::CropScale, new_scale);
    }

    pub fn crop_rotate_left(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.preset = crop.preset.rotate();
                    crop.rotation = (crop.rotation + 1) % 4;
                }
            });
    }

    pub fn crop_rotate_right(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.preset = crop.preset.rotate();
                    crop.rotation -= 1;
                    if crop.rotation < 0 {
                        crop.rotation = 3;
                    }
                }
            });
    }

    pub fn set_crop_preset(&mut self, crop_preset: CropPreset) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.preset = crop_preset;
                }
            });
    }

    pub fn toggle_parameters_visibility(&mut self) {
        self.parameters_visible = !self.parameters_visible;
    }

    pub fn toggle_favorite(&mut self) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                parameters.is_favorite = !parameters.is_favorite;
            });
    }

    pub fn white_balance_at(&mut self, x: i32, y: i32) {
        let lab_pixel: Option<LabPixel> = self.image.image.as_ref()
            .and_then(|opt| opt.lab_pixel_at(x as usize, y as usize));
        match lab_pixel {
            Some(pixel) => {
                self.image.parameter_history.lock().unwrap()
                    .update(|parameters| {
                        parameters.base_parameters.tint = -pixel.tint * 1000.0;
                        parameters.base_parameters.temperature = -pixel.temperature * 1000.0;
                    });
            },
            None => {}
        }
    }

    pub fn new_crop_offset_origin(&mut self, x: i32, y: i32) {
        self.mouse_origin_x = x;
        self.mouse_origin_y = y;
        if let Some(crop) = self.current_parameters().crop.as_ref() {
            self.offset_origin_x = crop.center_x;
            self.offset_origin_y = crop.center_y;
        }
    }

    pub fn update_crop_offset(&mut self, x: i32, y: i32) {
        let delta_x: i32 = self.mouse_origin_x - x;
        let delta_y: i32 = self.mouse_origin_y - y;
        let offset_x: i32 = self.offset_origin_x + delta_x;
        let offset_y: i32 = self.offset_origin_y + delta_y;
        self.update_crop(offset_x, offset_y);
    }

    pub fn update_crop(&mut self, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.center_x = x;
                    crop.center_y = y;
                }
            });
    }

    pub fn reset_view(&mut self) {
        self.image.image_view.lock().unwrap().reset();
    }

    pub fn update_view_zoom(&mut self, scroll_delta: f32) {
        self.image.image_view.lock().unwrap().update_zoom(scroll_delta * 0.05);
    }

    pub fn new_view_offset_origin(&mut self, x: i32, y: i32) {
        self.mouse_origin_x = x;
        self.mouse_origin_y = y;
        self.offset_origin_x = self.current_image_view().get_offset_x() as i32;
        self.offset_origin_y = self.current_image_view().get_offset_y() as i32;
    }

    pub fn update_view_offset(&mut self, x: i32, y: i32) {
        if let Some(crop) = self.image.parameter_history.lock().unwrap().current().crop {
            let view: ViewportCrop = crop.into();
            let image_width: i32 = (view.width as i32) / 2;
            let image_height: i32 = (view.height as i32) / 2;

            let delta_x: i32 = self.mouse_origin_x - x;
            let delta_y: i32 = self.mouse_origin_y - y;
            let offset_x: f32 = (self.offset_origin_x + delta_x).clamp(-image_width, image_width) as f32;
            let offset_y: f32 = (self.offset_origin_y + delta_y).clamp(-image_height, image_height) as f32;
            self.image.image_view.lock().unwrap().update_offset(offset_x, offset_y);
        }
    }

    pub fn current_view(&self) -> ViewportCrop {
        if let Some(current_crop) = self.image.parameter_history.lock().unwrap().current().crop {
            let view: ViewportCrop = current_crop.into();
            match self.view_mode {
                view_mode::ViewMode::Crop => {
                    let scale = 1.0;
                    ViewportCrop {
                        scale,
                        ..view
                    }
                },
                _ => {
                    let current_image_view = self.image.image_view.lock().unwrap();
                    let offset_x: i32 = current_image_view.get_offset_x() as i32;
                    let offset_y: i32 = current_image_view.get_offset_y() as i32;
                    let scale: f32 = 1.0 / (f32::powf(2.0, current_image_view.get_zoom()));
                    ViewportCrop {
                        center_x: view.center_x + offset_x,
                        center_y: view.center_y + offset_y,
                        width: view.width,
                        height: view.height,
                        angle_degrees: view.angle_degrees,
                        scale
                    }
                }
            }
        } else {
            ViewportCrop::default()
        }
    }

    fn set_parameter_value(&mut self, parameter: Parameter, new_value: f32) {
        self.image.parameter_history.lock().unwrap()
            .update_f32(parameter, |value| *value = new_value)
    }
}