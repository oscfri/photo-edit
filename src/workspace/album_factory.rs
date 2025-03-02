use std::sync::Arc;

use crate::repository::repository::Repository;

use super::{album::{Album, AlbumImage}, album_image_loader::AlbumImageLoader};

pub struct AlbumFactory {
    repository: Arc<Repository>,
    album_image_loader: Arc<AlbumImageLoader>
}

impl AlbumFactory {
    pub fn new(repository: Arc<Repository>, album_image_loader: Arc<AlbumImageLoader>) -> Self {
        Self { repository, album_image_loader }
    }

    pub fn create(&self) -> Album {
        let images: Vec<AlbumImage> = self.repository.get_album_photos().unwrap().iter()
            .map(|album_photo| self.album_image_loader.load(album_photo))
            .collect();
        
        Album {
            images: images
        }
    }
}