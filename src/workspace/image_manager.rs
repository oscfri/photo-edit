use std::{collections::BTreeMap, path::PathBuf, sync::{Arc, Mutex}};

use crate::{repository::repository::{AlbumPhotoDto, Repository}, types::RawImage};

use super::{album_image::AlbumImage, parameters::{Crop, ParameterHistory, Parameters}, workspace::{ImageView, WorkspaceImage}};

#[derive(Clone)]
struct SourceImage {
    path: PathBuf,
    image: Option<Arc<RawImage>>,
    thumbnail: Option<Arc<RawImage>>,
    parameter_history: Arc<Mutex<ParameterHistory>>,
    image_view: Arc<Mutex<ImageView>>,
}

pub struct ImagePath {
    pub photo_id: i32,
    pub path: PathBuf,
}

pub struct ImageManager {
    repository: Arc<Repository>,
    source_images: BTreeMap<i32, SourceImage>,
    is_filter_active: bool
}

impl ImageManager {
    fn new(
            repository: Arc<Repository>,
            source_images: BTreeMap<i32, SourceImage>) -> Self {
        Self {
            repository,
            source_images,
            is_filter_active: false
        }
    }

    pub fn create_from(repository: Arc<Repository>) -> Self {
        let source_images = Self::load_images(&repository);
        ImageManager::new(repository, source_images)
    }

    pub fn refresh(&mut self) {
        let mut new_images = Self::load_images(&self.repository);

        for (photo_id, source_image) in &self.source_images {
            if new_images.contains_key(photo_id) {
                new_images.insert(*photo_id, source_image.clone());
            }
        }

        self.source_images = new_images;
    }

    pub fn get_paths_without_image(&self) -> Vec<ImagePath> {
        self.source_images.iter()
            .filter(|(_, source_image)| source_image.image.is_none())
            .map(|(photo_id, source_image)| {
                let path = source_image.path.clone();
                ImagePath {
                    photo_id: *photo_id,
                    path
                }
            })
            .collect()
    }

    pub fn set_image(&mut self, photo_id: i32, image: RawImage, thumbnail: RawImage) {
        if let Some(source_image) = self.source_images.get_mut(&photo_id) {
            let image_width = image.width;
            let image_height = image.height;
            source_image.image = Some(Arc::new(image));
            source_image.thumbnail = Some(Arc::new(thumbnail));
            source_image.parameter_history.lock().unwrap()
                .update(|parameters| {
                    if parameters.crop.is_none() {
                        parameters.crop = Some(Self::create_default_crop(image_width, image_height));
                    }
                });
        }
    }

    pub fn get_all_album_images(&self) -> Vec<AlbumImage> {
        self.source_images.iter()
            .filter(|(_, image)| {
                if self.is_filter_active {
                    image.parameter_history.lock().unwrap().current().is_favorite
                } else {
                    true
                }
            })
            .map(|(photo_id, image)| {
                let thumbnail = image.thumbnail.clone();
                AlbumImage::new(*photo_id, thumbnail)
            })
            .collect()
    }

    pub fn get_workspace_image(&self, photo_id: i32) -> Option<WorkspaceImage> {
        self.source_images.get(&photo_id)
            .map(|image| {
                let file_name = image.path.file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or("default".into());
                WorkspaceImage::new(
                    photo_id,
                    image.image.clone(),
                    image.parameter_history.clone(),
                    image.image_view.clone(),
                    file_name)
            })
    }

    pub fn save(&self) {
        for (photo_id, image) in &self.source_images {
            let parameters = image.parameter_history.lock().unwrap().current();
            let parameters_str: String = serde_json::to_string(&parameters).ok().unwrap_or("{}".into());
            self.repository.save_photo_parameters(*photo_id, parameters_str).ok();
        }
    }

    pub fn delete_image(&mut self, photo_id: i32) {
        self.repository.delete_photo(photo_id).ok();
        self.source_images.remove(&photo_id);
    }

    pub fn toggle_filter(&mut self) {
        self.is_filter_active = !self.is_filter_active;
    }

    pub fn get_is_filter_active(&self) -> bool {
        self.is_filter_active
    }

    fn create_image(album_photo: &AlbumPhotoDto) -> SourceImage {
        let path = PathBuf::from(&album_photo.file_name);
        let image = None;
        let thumbnail = None;
        let paramters_raw = Self::parse_parameters(&album_photo.parameters);
        let parameter_history = Arc::new(Mutex::new(paramters_raw.into()));
        let image_view = Arc::new(Mutex::new(ImageView::default()));
        SourceImage {
            path,
            image,
            thumbnail,
            parameter_history,
            image_view
        }
    }

    fn parse_parameters(parameters: &String) -> Parameters {
        serde_json::from_str(&parameters).ok().unwrap_or(Parameters::default())
    }

    fn create_default_crop(image_width: usize, image_height: usize) -> Crop {
        Crop {
            center_x: (image_width as i32) / 2,
            center_y: (image_height as i32) / 2,
            width: image_width as i32,
            height: image_height as i32,
            angle_degrees: 0.0
        }
    }

    fn load_images(repository: &Arc<Repository>) -> BTreeMap<i32, SourceImage> {
        repository.get_album_photos().unwrap().iter()
            .map(|album_photo_dto| (album_photo_dto.id, Self::create_image(album_photo_dto)))
            .collect()
    }
}