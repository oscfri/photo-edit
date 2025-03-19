use core::f32;
use std::sync::{Arc, Mutex};

use crate::pipeline::export_image::export_image;
use crate::types::{LabPixel, RawImage};
use crate::ui::message::MouseState;
use crate::view_mode::ViewMode;
use crate::view_mode;

use super::parameters::{Crop, ParameterHistory, Parameters, RadialMask};

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

#[derive(Debug, Default, Clone)]
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

        if self.zoom < -1.0 {
            self.zoom = -1.0;
        } else if self.zoom > 10.0 {
            self.zoom = 10.0
        }
    }
}

#[derive(Clone)]
pub struct Workspace {
    image: WorkspaceImage,
    view_mode: ViewMode,
    parameters_visible: bool,

    // For view dragging (there's probably a better way to handle this)
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

    // TODO: Check all uses of this one
    pub fn current_parameters(&self) -> Parameters {
        self.image.parameter_history.lock().unwrap().current()
    }

    pub fn get_parameters_visible(&self) -> bool {
        self.parameters_visible
    }

    pub fn parameters_to_display(&self) -> Parameters {
        let parameters = self.image.parameter_history.lock().unwrap().current();

        if self.parameters_visible {
            parameters
        } else {
            Parameters {
                crop: parameters.crop,
                ..Parameters::default()
            }
        }
    }

    pub fn current_image_view(&self) -> ImageView {
        self.image.image_view.lock().unwrap().clone()
    }

    pub fn current_angle_degrees(&self) -> f32 {
        self.current_parameters().crop.as_ref()
            .map_or(0.0, |crop| crop.angle_degrees)
    }

    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode
    }

    pub fn get_mask_index(&self) -> Option<usize> {
        if let ViewMode::Mask(index) = self.view_mode {
            Some(index)
        } else {
            None
        }
    }

    pub fn export_image(&self) {
        futures_executor::block_on(export_image(&self));
    }

    pub fn undo(&mut self) {
        self.image.parameter_history.lock().unwrap().undo()
    }

    pub fn redo(&mut self) {
        self.image.parameter_history.lock().unwrap().redo()
    }

    pub fn toggle_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = self.view_mode.toggle_view_mode(view_mode);
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.brightness = brightness)
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.contrast = contrast)
    }

    pub fn set_tint(&mut self, tint: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.tint = tint)
    }
    
    pub fn set_temperature(&mut self, temperature: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.temperature = temperature)
    }
    
    pub fn set_saturation(&mut self, saturation: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| parameters.saturation = saturation)
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

    pub fn update_mask_radius(&mut self, mask_index: usize, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                let center_x = radial_mask.center_x;
                let center_y = radial_mask.center_y;
                radial_mask.width = (center_x - x).abs();
                radial_mask.height = (center_y - y).abs();
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

    pub fn set_mask_angle(&mut self, mask_index: usize, angle: f32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                let radial_mask = &mut parameters.radial_masks[mask_index];
                radial_mask.angle = angle;
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
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.angle_degrees = angle_degrees;
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
                        parameters.tint = -pixel.tint * 1000.0;
                        parameters.temperature = -pixel.temperature * 1000.0;
                    });
            },
            None => {}
        }
    }

    pub fn update_crop(&mut self, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    let width: f32 = (x - crop.center_x) as f32;
                    let height: f32 = (y - crop.center_y) as f32;
                    let angle: f32 = crop.angle_degrees / 180.0 * std::f32::consts::PI;
                    let sin: f32 = f32::sin(angle);
                    let cos: f32 = f32::cos(angle);
                    crop.width = ((width * cos + height * sin).abs() * 2.0) as i32;
                    crop.height = ((-width * sin + height * cos).abs() * 2.0) as i32;
                }
            });
    }

    pub fn new_crop(&mut self, x: i32, y: i32) {
        self.image.parameter_history.lock().unwrap()
            .update(|parameters| {
                if let Some(crop) = &mut parameters.crop {
                    crop.center_x = x;
                    crop.center_y = y;
                    crop.width = 0;
                    crop.height = 0;
                }
            });
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
        let delta_x: i32 = self.mouse_origin_x - x;
        let delta_y: i32 = self.mouse_origin_y - y;
        let offset_x: f32 = (self.offset_origin_x + delta_x) as f32;
        let offset_y: f32 = (self.offset_origin_y + delta_y) as f32;
        self.image.image_view.lock().unwrap().update_offset(offset_x, offset_y);
    }

    pub fn current_view(&self) -> Crop {
        match self.view_mode {
            // Show full image in Crop mode
            view_mode::ViewMode::Crop => Crop {
                center_x: (self.current_source_image().unwrap().width as i32) / 2,
                center_y: (self.current_source_image().unwrap().height as i32) / 2,
                width: self.current_source_image().unwrap().width as i32,
                height: self.current_source_image().unwrap().height as i32,
                angle_degrees: self.current_angle_degrees(),
            },
            _ => self.make_view()
        }
    }

    fn make_view(&self) -> Crop {
        if let Some(current_crop) = self.image.parameter_history.lock().unwrap().current().crop {
            let current_image_view = self.image.image_view.lock().unwrap();
            let scale: f32 = 1.0 / (f32::powf(2.0, current_image_view.get_zoom()));
            Crop {
                center_x: current_crop.center_x + (current_image_view.get_offset_x() as i32),
                center_y: current_crop.center_y + (current_image_view.get_offset_y() as i32),
                width: ((current_crop.width as f32) * scale) as i32,
                height: ((current_crop.height as f32) * scale) as i32,
                ..current_crop
            }
        } else {
            Crop::default()
        }
    }
}