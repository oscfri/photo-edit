use crate::album;
use crate::repository::repository::Repository;

use super::Workspace;

use std::path::PathBuf;

pub struct WorkspaceFactory<'a> {
    repository: &'a mut Repository
}

impl <'a> WorkspaceFactory<'a> {
    pub fn new(repository: &'a mut Repository) -> Self {
        Self { repository }
    }

    pub fn create(&mut self) -> Workspace {
        let file_paths = self.repository.get_album_photos(0).unwrap().iter()
            .map(|album_photo| album_photo.file_name.clone())
            .map(PathBuf::from)
            .collect();

        let album: album::Album = album::load_album(&file_paths);
        let image_index: usize = 0;
        Workspace::new(album, image_index)
    }
}