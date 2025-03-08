use std::{path::PathBuf, slice::Iter, sync::Arc};

use rayon::prelude::*;

use crate::{repository::repository::Repository, types::*};

use super::{parameters::{Crop, Parameters}, workspace::Workspace};

pub struct Album {
    repository: Arc<Repository>,
    pub images: Vec<AlbumImage>, // TODO: Avoid making this pub
    image_index: usize,
}

impl Album {
    pub fn new(repository: Arc<Repository>, images: Vec<AlbumImage>) -> Self {
        Self { repository, images, image_index: 0 }
    }

    pub fn iter_images(&self) -> Iter<AlbumImage> {
        self.images.iter()
    }

    pub fn update_workspace(&mut self, workspace: &Option<Workspace>) {
        if let Some(w) = workspace {
            self.images[self.image_index].parameters = w.get_parameters().clone()
        }
    }

    pub fn make_workspace(&self) -> Option<Workspace> {
        if self.images.is_empty() {
            None
        } else {
            let image = &self.images[self.image_index];
            Some(Workspace::new(
                image.source_image.clone(), // TODO: Allow empty image
                image.photo_id,
                image.parameters.clone(),
                image.image_view.clone()))
        }
    }

    pub fn save(&self) {
        for album_image in &self.images {
            let photo_id = album_image.photo_id;
            let parameters_str: String = serde_json::to_string(&album_image.parameters).ok().unwrap_or("{}".into());
            self.repository.save_photo_parameters(photo_id, parameters_str).ok();
        }
    }

    pub fn next_image(&mut self) {
        self.image_index = (self.image_index + 1) % self.images.len();
    }

    pub fn set_image_index(&mut self, index: usize) {
        if index < self.images.len() {
            self.image_index = index;
        }
    }

    pub fn set_image(&mut self, photo_id: i32, rgb_image: RgbImage) {
        for image in &mut self.images {
            if image.photo_id == photo_id {
                let raw_image = convert_to_raw_image(&rgb_image);
                let thumbnail = convert_to_raw_image(&resize_to_thumbnail_size(&rgb_image));
                image.source_image = Some(Arc::new(raw_image));
                image.thumbnail = Some(thumbnail);
                if image.parameters.crop.is_none() {
                    image.parameters.crop = Some(create_default_crop(rgb_image.width, rgb_image.height))
                }
                break
            }
        }
    }

    pub fn delete_image(&mut self) {
        if !self.images.is_empty() {
            let photo_id = self.images[self.image_index].photo_id;
            self.repository.delete_photo(photo_id).ok();
            self.images.remove(self.image_index);
            self.image_index = self.image_index.min((self.images.len() as i32 - 1).max(0) as usize);
        }
    }
}

fn create_default_crop(image_width: usize, image_height: usize) -> Crop {
    Crop {
        center_x: (image_width as i32) / 2,
        center_y: (image_height as i32) / 2,
        width: image_width as i32,
        height: image_height as i32,
        angle_degrees: 0.0,
    }
}

fn convert_to_raw_image(image: &RgbImage) -> RawImage {
    let mut buffer: Vec<u8> = vec![255; image.width * image.height * 4];

    buffer.par_iter_mut()
        .enumerate()
        .for_each(|(index, byte)| {
            let pixel_index: usize = index / 4;
            let channel_index: usize = index % 4;
            if channel_index == 0 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].red);
            } else if channel_index == 1 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].green);
            } else if channel_index == 2 {
                *byte = pixel_value_to_u8(image.pixels[pixel_index].blue);
            }
            // Don't bother with alpha, as it's 255 by default
        });

    RawImage {
        width: image.width,
        height: image.height,
        pixels: buffer
    }
}

fn pixel_value_to_u8(value: f32) -> u8 {
    if value <= 0.0 {
        0
    } else if value >= 1.0 {
        255
    } else {
        (value * 255.0) as u8
    }
}

fn resize_to_thumbnail_size(image: &RgbImage) -> RgbImage {
    let target_size: usize = 100;
    let width_skip: usize = std::cmp::max(1, image.width / target_size);
    let height_skip: usize = std::cmp::max(1, image.height / target_size);

    let target_width: usize = image.width / width_skip;
    let target_height: usize = image.height / height_skip;
    let mut pixels: Vec<RgbPixel> = Vec::new();

    for h in 0..target_height {
        for w in 0..target_width {
            pixels.push(image.pixels[(h * height_skip) * image.width + w * width_skip].clone());
        }
    }

    RgbImage {
        width: target_width,
        height: target_height,
        pixels: pixels
    }
}

pub struct AlbumImage {
    pub photo_id: i32,
    pub path: PathBuf,
    pub source_image: Option<Arc<RawImage>>,
    pub parameters: Parameters,
    pub image_view: ImageView,
    pub thumbnail: Option<RawImage>
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
