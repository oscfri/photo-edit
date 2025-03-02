use std::sync::Arc;

use crate::types::*;

use super::parameters::Parameters;

pub struct Album {
    pub images: Vec<AlbumImage>
}

pub struct AlbumImage {
    pub photo_id: i32,
    pub source_image: Arc<RawImage>,
    pub parameters: Parameters,
    pub image_view: ImageView,
    pub thumbnail: RawImage
}

impl AlbumImage {
    fn rgb_pixel_at(&self, x: usize, y: usize) -> Option<RgbPixel> {

        if x < self.source_image.width && y < self.source_image.height {
            let pixel_index: usize = (y * self.source_image.width + x) * 4; // Times 4 due to unused alpha channel
            let red: f32 = self.source_image.pixels[pixel_index + 0] as f32 / 255.0;
            let green: f32 = self.source_image.pixels[pixel_index + 1] as f32 / 255.0;
            let blue: f32 = self.source_image.pixels[pixel_index + 2] as f32 / 255.0;
            Some(RgbPixel { red, green, blue })
        } else {
            None
        }
    }

    pub fn lab_pixel_at(&self, x: usize, y: usize) -> Option<LabPixel> {
        self.rgb_pixel_at(x, y).map(rgb_pixel_to_lab)
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

        if self.zoom < 0.0 {
            self.zoom = 0.0;
        } else if self.zoom > 10.0 {
            self.zoom = 10.0
        }
    }
}
