pub mod workspace_factory;

use crate::album::{self, AlbumImage};
use crate::view_mode::ViewMode;
use crate::{types, view_mode};

pub struct Workspace {
    album: album::Album,
    image_index: usize,
    view_mode: ViewMode,
    has_updated: bool
}

fn calculate_distance(x1: i32, y1: i32, x2: i32, y2: i32) -> f32 {
    (((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)) as f32).sqrt()
}

impl Workspace {
    pub fn new(album: album::Album, image_index: usize) -> Self {
        Self {
            album,
            image_index,
            view_mode: ViewMode::Normal,
            has_updated: true
        }
    }

    pub fn get_has_updated(&self) -> bool {
        self.has_updated
    }

    fn set_has_updated(&mut self) {
        self.has_updated = true;
    }

    pub fn reset_has_updated(&mut self) {
        self.has_updated = false;
    }

    pub fn get_image_index(&self) -> usize {
        self.image_index
    }

    pub fn album_images(&self) -> &Vec<album::AlbumImage> {
        &self.album.images
    }

    pub fn current_image(&self) -> &AlbumImage {
        &self.album.images[self.image_index]
    }

    fn current_image_mut(&mut self) -> &mut AlbumImage {
        self.set_has_updated(); // If this has been called, then an update is probably needed
        &mut self.album.images[self.image_index]
    }

    pub fn current_source_image(&self) -> &types::RawImage {
        &self.current_image().source_image
    }

    pub fn current_parameters(&self) -> &album::Parameters {
        &self.current_image().parameters
    }

    fn current_parameters_mut(&mut self) -> &mut album::Parameters {
        self.set_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_image_mut().parameters
    }

    pub fn current_image_view(&self) -> &album::ImageView {
        &self.current_image().image_view
    }

    pub fn current_image_view_mut(&mut self) -> &mut album::ImageView {
        self.set_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_image_mut().image_view
    }

    pub fn current_crop(&self) -> &album::Crop {
        &self.current_image().crop
    }

    fn current_crop_mut(&mut self) -> &mut album::Crop {
        self.set_has_updated(); // If this has been called, then an update is probably needed
        &mut self.current_image_mut().crop
    }

    pub fn get_view_mode(&self) -> ViewMode {
        self.view_mode
    }

    pub fn toggle_view_mode(&mut self, view_mode: ViewMode) {
        self.set_has_updated();
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
        current_parameters.radial_masks.push(album::RadialMask::default());
        self.view_mode = ViewMode::Mask(new_mask_index);
    }

    pub fn delete_mask(&mut self, mask_index: usize) {
        self.current_parameters_mut().radial_masks.remove(mask_index);
        self.view_mode = ViewMode::Normal;
    }

    pub fn update_mask_position(&mut self, mask_index: usize, x: i32, y: i32) {
        let parameters: &mut album::Parameters = self.current_parameters_mut();
        let radial_mask: &mut album::RadialMask = &mut parameters.radial_masks[mask_index];
        radial_mask.center_x = x;
        radial_mask.center_y = y;
        radial_mask.radius = 0.0;
    }

    pub fn update_mask_radius(&mut self, mask_index: usize, x: i32, y: i32) {
        let parameters: &mut album::Parameters = self.current_parameters_mut();
        let radial_mask: &mut album::RadialMask = &mut parameters.radial_masks[mask_index];
        let center_x = radial_mask.center_x;
        let center_y = radial_mask.center_y;
        radial_mask.radius = calculate_distance(center_x, center_y, x, y);
    }

    pub fn set_mask_brightness(&mut self, mask_index: usize, brightness: f32) {
        self.current_parameters_mut().radial_masks[mask_index].brightness = brightness;
    }

    pub fn set_crop_angle(&mut self, angle_degrees: f32) {
        self.current_crop_mut().angle_degrees = angle_degrees;
    }

    pub fn white_balance_at(&mut self, x: i32, y: i32) {
        let current_image: &album::AlbumImage = self.current_image();
        match current_image.lab_pixel_at(x as usize, y as usize) {
            Some(pixel) => {
                let parameters: &mut album::Parameters = self.current_parameters_mut();
                parameters.tint = -pixel.tint;
                parameters.temperature = -pixel.temperature;
            },
            None => {}
        }
    }

    pub fn update_crop(&mut self, x: i32, y: i32) {
        let crop: &mut album::Crop = self.current_crop_mut();
        let width: f32 = (x - crop.center_x) as f32;
        let height: f32 = (y - crop.center_y) as f32;
        let angle: f32 = crop.angle_degrees / 180.0 * std::f32::consts::PI;
        let sin: f32 = f32::sin(angle);
        let cos: f32 = f32::cos(angle);
        crop.width = ((width * cos + height * sin).abs() * 2.0) as i32;
        crop.height = ((-width * sin + height * cos).abs() * 2.0) as i32;
    }

    pub fn new_crop(&mut self, x: i32, y: i32) {
        let crop: &mut album::Crop = self.current_crop_mut();
        crop.center_x = x;
        crop.center_y = y;
        crop.width = 0;
        crop.height = 0;
    }

    pub fn update_zoom(&mut self, scroll_delta: f32) {
        self.current_image_view_mut().update_zoom(scroll_delta * 0.05);
    }

    pub fn current_view(&self) -> album::Crop {
        match self.view_mode {
            // Show full image in Crop mode
            view_mode::ViewMode::Crop => album::Crop {
                center_x: (self.current_source_image().width as i32) / 2,
                center_y: (self.current_source_image().height as i32) / 2,
                width: self.current_source_image().width as i32,
                height: self.current_source_image().height as i32,
                angle_degrees: self.current_crop().angle_degrees,
            },
            _ => self.make_view()
        }
    }

    fn make_view(&self) -> album::Crop {
        let current_crop: &album::Crop = self.current_crop();
        let scale: f32 = 1.0 / (f32::powf(2.0, self.current_image_view().get_zoom()));
        album::Crop {
            center_x: current_crop.center_x,
            center_y: current_crop.center_y,
            width: ((current_crop.width as f32) * scale) as i32,
            height: ((current_crop.height as f32) * scale) as i32,
            angle_degrees: current_crop.angle_degrees
        }
    }

    pub fn next_image_index(&mut self) {
        self.image_index = (self.image_index + 1) % self.album.images.len();
    }

    pub fn set_image_index(&mut self, index: usize) {
        if index < self.album.images.len() {
            self.image_index = index;
        }
    }
}