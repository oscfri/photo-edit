use core::f32;

use crate::pipeline::export_image::export_image;
use crate::repository::repository::Repository;
use crate::view_mode::ViewMode;
use crate::{types, view_mode};

use super::album::{Album, AlbumImage, ImageView};
use super::parameters::{Crop, Parameters, RadialMask};

pub struct Workspace {
    album: Album,
    image_index: usize,
    view_mode: ViewMode,
    has_updated: bool,

    // For view dragging (there's probably a better way to handle this)
    mouse_origin_x: i32,
    mouse_origin_y: i32,
    offset_origin_x: i32,
    offset_origin_y: i32,
}

impl Workspace {
    pub fn new(album: Album) -> Self {
        let image_index = 0;
        Self {
            album,
            image_index,
            view_mode: ViewMode::Normal,
            has_updated: true,
            mouse_origin_x: 0,
            mouse_origin_y: 0,
            offset_origin_x: 0,
            offset_origin_y: 0,
        }
    }

    pub fn get_has_updated(&self) -> bool {
        self.has_updated
    }

    fn mark_has_updated(&mut self) {
        self.has_updated = true;
    }

    pub fn reset_has_updated(&mut self) {
        self.has_updated = false;
    }

    pub fn get_image_index(&self) -> usize {
        self.image_index
    }

    pub fn album_images(&self) -> &Vec<AlbumImage> {
        &self.album.images
    }

    pub fn current_image(&self) -> &AlbumImage {
        &self.album.images[self.image_index]
    }

    fn current_image_mut(&mut self) -> &mut AlbumImage {
        self.mark_has_updated(); // If this has been called, then an update is probably needed
        &mut self.album.images[self.image_index]
    }

    pub fn current_source_image(&self) -> &types::RawImage {
        &self.current_image().source_image
    }

    pub fn current_parameters(&self) -> &Parameters {
        &self.current_image().parameters
    }

    fn current_parameters_mut(&mut self) -> &mut Parameters {
        self.mark_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_image_mut().parameters
    }

    pub fn current_image_view(&self) -> &ImageView {
        &self.current_image().image_view
    }

    fn current_image_view_mut(&mut self) -> &mut ImageView {
        self.mark_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_image_mut().image_view
    }

    pub fn current_crop(&self) -> &Crop {
        &self.current_parameters().crop
    }

    fn current_crop_mut(&mut self) -> &mut Crop {
        self.mark_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_parameters_mut().crop
    }

    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode
    }

    pub fn save_album(&self, repository: &Repository) {
        for album_image in &self.album.images {
            let photo_id = album_image.photo_id;
            let parameters_str: String = serde_json::to_string(&album_image.parameters).ok().unwrap_or("{}".into());
            repository.save_photo_parameters(photo_id, parameters_str).ok();
        }
    }

    pub fn export_image(&self) {
        futures_executor::block_on(export_image(&self));
    }

    pub fn toggle_view_mode(&mut self, view_mode: ViewMode) {
        self.mark_has_updated();
        self.view_mode = self.view_mode.toggle_view_mode(view_mode);
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        self.current_parameters_mut().brightness = brightness
    }

    pub fn set_contrast(&mut self, contrast: f32) {
        self.current_parameters_mut().contrast = contrast
    }

    pub fn set_tint(&mut self, tint: f32) {
        self.current_parameters_mut().tint = tint;
    }
    
    pub fn set_temperature(&mut self, temperature: f32) {
        self.current_parameters_mut().temperature = temperature;
    }
    
    pub fn set_saturation(&mut self, saturation: f32) {
        self.current_parameters_mut().saturation = saturation;
    }

    pub fn add_mask(&mut self) {
        let current_parameters = self.current_parameters_mut();
        let new_mask_index = current_parameters.radial_masks.len();
        current_parameters.radial_masks.push(RadialMask::default());
        self.view_mode = ViewMode::Mask(new_mask_index);
    }

    pub fn delete_mask(&mut self, mask_index: usize) {
        self.current_parameters_mut().radial_masks.remove(mask_index);
        self.view_mode = ViewMode::Normal;
    }

    pub fn update_mask_position(&mut self, mask_index: usize, x: i32, y: i32) {
        let parameters: &mut Parameters = self.current_parameters_mut();
        let radial_mask: &mut RadialMask = &mut parameters.radial_masks[mask_index];
        radial_mask.center_x = x;
        radial_mask.center_y = y;
        radial_mask.width = 0;
        radial_mask.height = 0;
    }

    pub fn update_mask_radius(&mut self, mask_index: usize, x: i32, y: i32) {
        let parameters: &mut Parameters = self.current_parameters_mut();
        let radial_mask: &mut RadialMask = &mut parameters.radial_masks[mask_index];
        let center_x = radial_mask.center_x;
        let center_y = radial_mask.center_y;
        radial_mask.width = (center_x - x).abs();
        radial_mask.height = (center_y - y).abs();
    }

    pub fn set_mask_is_linear(&mut self, mask_index: usize, is_linear: bool) {
        self.current_parameters_mut().radial_masks[mask_index].is_linear = is_linear;
    }

    pub fn set_mask_brightness(&mut self, mask_index: usize, brightness: f32) {
        self.current_parameters_mut().radial_masks[mask_index].brightness = brightness;
    }

    pub fn set_mask_angle(&mut self, mask_index: usize, angle: f32) {
        let parameters: &mut Parameters = self.current_parameters_mut();
        let radial_mask: &mut RadialMask = &mut parameters.radial_masks[mask_index];
        radial_mask.angle = angle;
    }

    pub fn set_crop_angle(&mut self, angle_degrees: f32) {
        self.current_crop_mut().angle_degrees = angle_degrees;
    }

    pub fn white_balance_at(&mut self, x: i32, y: i32) {
        let current_image: &AlbumImage = self.current_image();
        match current_image.lab_pixel_at(x as usize, y as usize) {
            Some(pixel) => {
                let parameters: &mut Parameters = self.current_parameters_mut();
                parameters.tint = -pixel.tint * 1000.0;
                parameters.temperature = -pixel.temperature * 1000.0;
            },
            None => {}
        }
    }

    pub fn update_crop(&mut self, x: i32, y: i32) {
        let crop: &mut Crop = self.current_crop_mut();
        let width: f32 = (x - crop.center_x) as f32;
        let height: f32 = (y - crop.center_y) as f32;
        let angle: f32 = crop.angle_degrees / 180.0 * std::f32::consts::PI;
        let sin: f32 = f32::sin(angle);
        let cos: f32 = f32::cos(angle);
        crop.width = ((width * cos + height * sin).abs() * 2.0) as i32;
        crop.height = ((-width * sin + height * cos).abs() * 2.0) as i32;
    }

    pub fn new_crop(&mut self, x: i32, y: i32) {
        let crop: &mut Crop = self.current_crop_mut();
        crop.center_x = x;
        crop.center_y = y;
        crop.width = 0;
        crop.height = 0;
    }

    pub fn update_view_zoom(&mut self, scroll_delta: f32) {
        self.current_image_view_mut().update_zoom(scroll_delta * 0.05);
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
        self.current_image_view_mut().update_offset(offset_x, offset_y);
    }

    pub fn current_view(&self) -> Crop {
        match self.view_mode {
            // Show full image in Crop mode
            view_mode::ViewMode::Crop => Crop {
                center_x: (self.current_source_image().width as i32) / 2,
                center_y: (self.current_source_image().height as i32) / 2,
                width: self.current_source_image().width as i32,
                height: self.current_source_image().height as i32,
                angle_degrees: self.current_crop().angle_degrees,
            },
            _ => self.make_view()
        }
    }

    fn make_view(&self) -> Crop {
        let current_crop: &Crop = self.current_crop();
        let current_image_view: &ImageView = self.current_image_view();
        let scale: f32 = 1.0 / (f32::powf(2.0, current_image_view.get_zoom()));
        Crop {
            center_x: current_crop.center_x + (current_image_view.get_offset_x() as i32),
            center_y: current_crop.center_y + (current_image_view.get_offset_y() as i32),
            width: ((current_crop.width as f32) * scale) as i32,
            height: ((current_crop.height as f32) * scale) as i32,
            ..current_crop.clone()
        }
    }

    pub fn next_image_index(&mut self) {
        self.mark_has_updated();

        self.image_index = (self.image_index + 1) % self.album.images.len();
    }

    pub fn set_image_index(&mut self, index: usize) {
        self.mark_has_updated();
        
        if index < self.album.images.len() {
            self.image_index = index;
        }
    }
}