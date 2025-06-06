use std::{collections::BTreeMap, path::PathBuf, sync::{Arc, Mutex}};

use itertools::Itertools;

use crate::{repository::album_repository::{AlbumPhotoDto, AlbumRepository}, types::RawImage};

use super::{album_image::AlbumImage, parameters::{Crop, CropPreset, ParameterHistory, Parameters}, workspace::{ImageView, WorkspaceImage}};

// TODO: Adapt these values
const CACHE_SIZE: usize = 20;
const LOAD_SIZE: usize = 5;

#[derive(Clone)]
struct SourceImage {
    path: PathBuf,
    image: Option<Arc<RawImage>>,
    thumbnail: Option<Arc<RawImage>>,
    parameter_history: Arc<Mutex<ParameterHistory>>,
    image_view: Arc<Mutex<ImageView>>,
    pending_load: bool
}

pub struct ImagePathToLoad {
    pub photo_id: i32,
    pub path: PathBuf,
}

pub struct ImageManager {
    repository: Arc<AlbumRepository>,
    source_images: BTreeMap<i32, SourceImage>,
    is_filter_active: bool
}

impl ImageManager {
    fn new(
            repository: Arc<AlbumRepository>,
            source_images: BTreeMap<i32, SourceImage>) -> Self {
        Self {
            repository,
            source_images,
            is_filter_active: false
        }
    }

    pub fn create_from(repository: Arc<AlbumRepository>) -> Self {
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

    pub fn get_paths_to_load(&self, photo_id_hint: i32) -> Vec<ImagePathToLoad> {
        let photo_ids_to_load: Vec<i32> = self.get_photo_ids_to_have_in_memory(photo_id_hint, LOAD_SIZE);
        
        photo_ids_to_load.iter()
            .flat_map(|photo_id| self.source_images.get(&photo_id)
                .map(|source_image| (photo_id, source_image)))
            .filter(|(_, source_image)| source_image.image.is_none())
            .filter(|(_, source_image)| !source_image.pending_load)
            .map(|(photo_id, source_image)| {
                let path = source_image.path.clone();
                ImagePathToLoad {
                    photo_id: *photo_id,
                    path
                }
            })
            .collect()
    }

    pub fn set_image(&mut self, photo_id: i32, image: RawImage, thumbnail: RawImage) {
        if let Some(source_image) = self.source_images.get_mut(&photo_id) {
            self.repository.add_thumbnail(photo_id, &thumbnail).ok();

            let image_width = image.width;
            let image_height = image.height;
            source_image.pending_load = false;
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

    pub fn set_image_pending_load(&mut self, photo_id: i32) {
        if let Some(source_image) = self.source_images.get_mut(&photo_id) {
            source_image.pending_load = true
        }
    }

    pub fn flush_cache(&mut self, photo_id_hint: i32) {
        let photo_ids_to_keep: Vec<i32> = self.get_photo_ids_to_have_in_memory(photo_id_hint, CACHE_SIZE);

        let photo_ids_to_unload: Vec<i32> = self.source_images.iter()
            .map(|(photo_id, _)| *photo_id)
            .filter(|photo_id| !photo_ids_to_keep.contains(photo_id))
            .collect();

        for photo_id in photo_ids_to_unload {
            if let Some(source_image) = self.source_images.get_mut(&photo_id) {
                source_image.image = None;
            }
        }
    }

    pub fn get_all_album_images(&self) -> Vec<AlbumImage> {
        self.source_images.iter()
            .filter(|(_, image)| self.should_display_image(&image))
            .map(|(photo_id, image)| {
                let thumbnail = image.thumbnail.clone();
                AlbumImage::new(*photo_id, thumbnail)
            })
            .collect()
    }

    pub fn get_workspace_image(&self, photo_id: i32) -> Option<WorkspaceImage> {
        self.source_images.get(&photo_id)
            .map(|source_image| {
                let file_name = source_image.path.file_stem()
                    .map(|s| s.to_string_lossy().into_owned())
                    .unwrap_or("default".into());
                WorkspaceImage::new(
                    photo_id,
                    source_image.image.clone(),
                    source_image.parameter_history.clone(),
                    source_image.image_view.clone(),
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
        let thumbnail = album_photo.thumbnail.as_ref()
            .map(|thumbnail| Arc::new(thumbnail.clone()));
        let paramters_raw = Self::parse_parameters(&album_photo.parameters);
        let parameter_history = Arc::new(Mutex::new(paramters_raw.into()));
        let image_view = Arc::new(Mutex::new(ImageView::default()));
        let pending_load = false;


        SourceImage {
            path,
            image,
            thumbnail,
            parameter_history,
            image_view,
            pending_load
        }
    }

    fn parse_parameters(parameters: &String) -> Parameters {
        serde_json::from_str(&parameters).ok().unwrap_or(Parameters::default())
    }

    fn create_default_crop(image_width: usize, image_height: usize) -> Crop {
        Crop {
            center_x: (image_width as i32) / 2,
            center_y: (image_height as i32) / 2,
            source_image_width: image_width,
            source_image_height: image_height,
            scale: 0.0,
            angle_degrees: 0.0,
            rotation: 0,
            preset: CropPreset::Original
        }
    }

    fn load_images(repository: &Arc<AlbumRepository>) -> BTreeMap<i32, SourceImage> {
        repository.get_album_photos().unwrap().iter()
            .map(|album_photo_dto| (album_photo_dto.id, Self::create_image(album_photo_dto)))
            .collect()
    }

    fn should_display_image(&self, source_image: &SourceImage) -> bool {
        if self.is_filter_active {
            source_image.parameter_history.lock().unwrap().current().is_favorite
        } else {
            true
        }
    }

    fn get_photo_ids_to_have_in_memory(&self, photo_id_hint: i32, cache_size: usize) -> Vec<i32> {
        let photo_ids_to_display: Vec<i32> = self.source_images.iter()
            .filter(|(_, source_image)| self.should_display_image(source_image))
            .map(|(photo_id, _)| *photo_id)
            .collect();

        let index_hint = photo_ids_to_display.iter().enumerate()
            .min_by_key(|(_, photo_id)| (*photo_id - photo_id_hint).abs())
            .map(|(index, _)| index as i32)
            .unwrap_or(0);

        photo_ids_to_display.iter().enumerate()
            .sorted_by_key(|(index, _)| ((*index as i32) - index_hint).abs())
            .map(|(_, photo_id)| *photo_id)
            .take(cache_size)
            .collect()
    }
}