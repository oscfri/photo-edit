use crate::types::*;

pub struct Album {
    pub images: Vec<AlbumImage>
}

pub struct AlbumImage {
    pub photo_id: i32,
    pub source_image: RawImage,
    pub parameters: Parameters,
    pub image_view: ImageView,
    pub crop: Crop,
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

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct Parameters {
    pub brightness: f32,
    pub contrast: f32,
    pub tint: f32,
    pub temperature: f32,
    pub saturation: f32,
    pub radial_masks: Vec<RadialMask>
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
pub struct RadialMask {
    pub center_x: i32,
    pub center_y: i32,
    pub radius: f32,
    pub brightness: f32,
}

#[derive(Debug, Default, Clone)]
pub struct Crop {
    pub center_x: i32,
    pub center_y: i32,
    pub width: i32,
    pub height: i32,
    pub angle_degrees: f32,
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
