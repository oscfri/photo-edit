use std::sync::Arc;

use crate::types::RawImage;

pub struct AlbumImage {
    pub photo_id: i32,
    pub thumbnail: Option<Arc<RawImage>>
}

impl AlbumImage {
    pub fn new(
            photo_id: i32,
            thumbnail: Option<Arc<RawImage>>) -> Self {
        Self { photo_id, thumbnail }
    }
}