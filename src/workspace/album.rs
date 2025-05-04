use itertools::Itertools;

use super::album_image::AlbumImage;

pub struct Album {
    images: Vec<AlbumImage>,
    image_index: usize,
}

impl Album {
    pub fn new(images: Vec<AlbumImage>) -> Self {
        Self { images, image_index: 0 }
    }

    pub fn set_images(&mut self, images: Vec<AlbumImage>) {
        let current_photo_id = self.images.get(self.image_index)
            .map(|image| image.photo_id)
            .unwrap_or(0);

        self.images = images;
        self.image_index = self.images.iter()
            .enumerate()
            .find_or_last(|(_, image)| image.photo_id >= current_photo_id)
            .map(|(index, _)| index)
            .unwrap_or(0);
    }

    pub fn get_images(&self) -> &Vec<AlbumImage> {
        &self.images
    }

    pub fn get_image_index(&self) -> usize {
        self.image_index
    }

    pub fn get_photo_id(&self) -> Option<i32> {
        self.images.get(self.image_index)
            .map(|image| image.photo_id)
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
}