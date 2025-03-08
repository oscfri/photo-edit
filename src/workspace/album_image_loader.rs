use std::path::PathBuf;

use crate::repository::repository::AlbumPhoto;
use crate::workspace::album::{AlbumImage, ImageView};

use super::parameters::Parameters;

pub struct AlbumImageLoader {
}

// TODO: Should be renamed
impl AlbumImageLoader {
    pub fn new() -> Self {
        Self { }
    }

    pub fn create_from(&self, album_photo: &AlbumPhoto) -> AlbumImage {
        let photo_id = album_photo.id;
        let path = PathBuf::from(&album_photo.file_name);
        let source_image = None;
        let parameters = self.parse_parameters(&album_photo.parameters);
        let image_view = ImageView {
            offset_x: 0.0,
            offset_y: 0.0,
            zoom: 0.0
        };
        // let thumbnail: RawImage = self.convert_to_thumbnail(&rgb_image);
        let thumbnail = None; 
        AlbumImage {
            photo_id,
            path,
            source_image,
            parameters,
            image_view,
            thumbnail,
        }
    }

    fn parse_parameters(&self, parameters: &String) -> Parameters {
        serde_json::from_str(&parameters).ok().unwrap_or(Parameters::default())
    }
}