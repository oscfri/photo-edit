use crate::album::{Album, AlbumImage};
use crate::repository::repository::Repository;

use super::album_image_loader::AlbumImageLoader;

pub struct AlbumFactory<'a> {
    repository: &'a Repository,
    album_image_loader: &'a AlbumImageLoader
}

impl <'a> AlbumFactory<'a> {
    pub fn new(repository: &'a Repository, album_image_loader: &'a AlbumImageLoader) -> Self {
        Self { repository, album_image_loader }
    }

    pub fn create(&mut self) -> Album {
        let images: Vec<AlbumImage> = self.repository.get_album_photos(0).unwrap().iter()
            .map(|album_photo| self.album_image_loader.load(album_photo))
            .collect();
        
        Album {
            images: images
        }
    }
}