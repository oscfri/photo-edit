use std::sync::Arc;

use super::{album::Album, album_factory::AlbumFactory, workspace::Workspace};

pub struct WorkspaceFactory {
    album_factory: Arc<AlbumFactory>
}

impl WorkspaceFactory {
    pub fn new(album_factory: Arc<AlbumFactory>) -> Self {
        Self { album_factory }
    }

    pub fn create(&self) -> Workspace {
        let album: Album = self.album_factory.create();
        
        Workspace::new(album)
    }
}