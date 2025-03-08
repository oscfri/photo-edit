use std::{path::PathBuf, slice::Iter, sync::Arc};

use crate::{repository::repository::Repository, types::*};

use super::{parameters::{Crop, Parameters}, workspace::Workspace};

pub struct Album {
    repository: Arc<Repository>,
    images: Vec<AlbumImage>,
    image_index: usize,
}

impl Album {
    pub fn new(repository: Arc<Repository>, images: Vec<AlbumImage>) -> Self {
        Self { repository, images, image_index: 0 }
    }

    pub fn get_images(&self) -> &Vec<AlbumImage> {
        &self.images
    }

    pub fn get_image_index(&self) -> usize {
        self.image_index
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

    pub fn previous_image(&mut self) {
        self.image_index = (self.images.len() + self.image_index - 1) % self.images.len();
    }

    pub fn set_image_index(&mut self, index: usize) {
        if index < self.images.len() {
            self.image_index = index;
        }
    }

    pub fn set_image(&mut self, photo_id: i32, raw_image: RawImage, thumbnail: RawImage) {
        for image in &mut self.images {
            if image.photo_id == photo_id {
                if image.parameters.crop.is_none() {
                    image.parameters.crop = Some(create_default_crop(raw_image.width, raw_image.height))
                }
                image.source_image = Some(Arc::new(raw_image));
                image.thumbnail = Some(thumbnail);
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
