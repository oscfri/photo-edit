use crate::album;
use crate::pipeline::viewport;
use crate::types;

use std::path::PathBuf;

pub struct WorkSpace {
    album: album::Album,
    image_index: usize
}

pub fn load_workspace(file_paths: &Vec<PathBuf>) -> WorkSpace {
    let album: album::Album = album::load_album(file_paths);
    let image_index: usize = 0;
    WorkSpace::new(album, image_index)
}

impl WorkSpace {
    pub fn new(album: album::Album, image_index: usize) -> Self {
        Self {
            album,
            image_index
        }
    }

    pub fn album_images(&self) -> &Vec<album::AlbumImage> {
        &self.album.images
    }

    pub fn current_image(&self) -> &types::RawImage {
        &self.album.images[self.image_index].source_image
    }

    pub fn current_parameters(&self) -> &album::Parameters {
        &self.album.images[self.image_index].parameters
    }

    pub fn current_parameters_mut(&mut self) -> &mut album::Parameters {
        &mut self.album.images[self.image_index].parameters
    }

    pub fn current_crop(&self) -> &album::Crop {
        &self.album.images[self.image_index].crop
    }

    pub fn make_viewport(&self) -> viewport::Viewport {
        viewport::Viewport {
            image: self.current_image().clone(),
            image_index: self.image_index,
            parameters: self.current_parameters().clone(),
            crop: self.current_crop().clone()
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