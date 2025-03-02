use std::sync::Arc;

use crate::{repository::repository::Repository, types::*};

use super::{parameters::Parameters, workspace::Workspace};

pub struct Album {
    repository: Arc<Repository>,
    pub images: Vec<AlbumImage>, // TODO: Avoid making this pub
    image_index: usize,
}

impl Album {
    pub fn new(repository: Arc<Repository>, images: Vec<AlbumImage>) -> Self {
        Self { repository, images, image_index: 0 }
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
                image.source_image.clone(),
                self.image_index,
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

    pub fn set_image(&mut self, index: usize) {
        if index < self.images.len() {
            self.image_index = index;
        }
    }

    pub fn delete_image(&mut self) {
        if !self.images.is_empty() {
            let photo_id = self.images[self.image_index].photo_id;
            self.repository.delete_photo(photo_id).ok();
            self.images.remove(self.image_index);
            self.image_index = self.image_index.min(self.images.len());
        }
    }
}

pub struct AlbumImage {
    pub photo_id: i32,
    pub source_image: Arc<RawImage>,
    pub parameters: Parameters,
    pub image_view: ImageView,
    pub thumbnail: RawImage
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
