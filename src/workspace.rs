use crate::{album::{Album, AlbumImage, Crop, Parameters}, pipeline::viewport::Viewport, types::RawImage};

pub struct WorkSpace {
    album: Album,
    image_index: usize
}

impl WorkSpace {
    pub fn new(album: Album, image_index: usize) -> Self {
        Self {
            album,
            image_index
        }
    }

    pub fn album_images(&self) -> &Vec<AlbumImage> {
        &self.album.images
    }

    pub fn current_image(&self) -> &RawImage {
        &self.album.images[self.image_index].source_image
    }

    pub fn current_parameters(&self) -> &Parameters {
        &self.album.images[self.image_index].parameters
    }

    pub fn current_parameters_mut(&mut self) -> &mut Parameters {
        &mut self.album.images[self.image_index].parameters
    }

    pub fn current_crop(&self) -> &Crop {
        &self.album.images[self.image_index].crop
    }

    pub fn make_viewport(&self) -> Viewport {
        Viewport {
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