use crate::album::{self, AlbumImage};
use crate::pipeline::viewport;
use crate::{types, view_mode};

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

    pub fn current_image(&self) -> &AlbumImage {
        &self.album.images[self.image_index]
    }

    fn current_image_mut(&mut self) -> &mut AlbumImage {
        &mut self.album.images[self.image_index]
    }

    pub fn current_source_image(&self) -> &types::RawImage {
        &self.current_image().source_image
    }

    pub fn current_parameters(&self) -> &album::Parameters {
        &self.current_image().parameters
    }

    pub fn current_parameters_mut(&mut self) -> &mut album::Parameters {
        &mut self.current_image_mut().parameters
    }

    pub fn current_crop(&self) -> &album::Crop {
        &self.current_image().crop
    }

    pub fn current_crop_mut(&mut self) -> &mut album::Crop {
        &mut self.current_image_mut().crop
    }

    pub fn make_viewport(&self, view_mode: &view_mode::ViewMode) -> viewport::ViewportWorkspace {
        let view: album::Crop = self.current_view(view_mode);
        viewport::ViewportWorkspace::new(
            self.current_source_image().clone(),
            self.image_index,
            self.current_parameters().clone(),
            self.current_crop().clone(),
            view)
    }

    pub fn current_view(&self, view_mode: &view_mode::ViewMode) -> album::Crop {
        match view_mode {
            view_mode::ViewMode::Crop => album::Crop {
                x1: 0,
                x2: self.current_source_image().width as i32,
                y1: 0,
                y2: self.current_source_image().height as i32,
                degrees_angle: self.current_crop().degrees_angle,
            },
            view_mode::ViewMode::Normal => self.current_crop().clone()
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