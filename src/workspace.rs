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

    pub fn current_image_view(&self) -> &album::ImageView {
        &self.current_image().image_view
    }

    pub fn current_image_view_mut(&mut self) -> &mut album::ImageView {
        &mut self.current_image_mut().image_view
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
            // Show full image in Crop mode
            view_mode::ViewMode::Crop => album::Crop {
                center_x: (self.current_source_image().width as i32) / 2,
                center_y: (self.current_source_image().height as i32) / 2,
                width: self.current_source_image().width as i32,
                height: self.current_source_image().height as i32,
                angle_degrees: self.current_crop().angle_degrees,
            },
            view_mode::ViewMode::Normal | view_mode::ViewMode::Mask(_) => self.make_view()
        }
    }

    fn make_view(&self) -> album::Crop {
        let current_crop: &album::Crop = self.current_crop();
        let scale: f32 = 1.0 / (f32::powf(2.0, self.current_image_view().get_zoom()));
        album::Crop {
            center_x: current_crop.center_x,
            center_y: current_crop.center_y,
            width: ((current_crop.width as f32) * scale) as i32,
            height: ((current_crop.height as f32) * scale) as i32,
            angle_degrees: current_crop.angle_degrees
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