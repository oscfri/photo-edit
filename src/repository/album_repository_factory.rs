use std::sync::Arc;

use rusqlite::{Connection, Result};

use super::album_repository::AlbumRepository;

pub struct AlbumRepositoryFactory {
    connection: Arc<Connection>
}

impl<'a> AlbumRepositoryFactory {
    pub fn new(connection: Arc<Connection>) -> Self {
        Self { connection }
    }

    pub fn create(self) -> AlbumRepository {
        self.create_photo_table().unwrap();
        self.create_thumbnail_table().unwrap();
        AlbumRepository::new(self.connection)
    }

    fn create_photo_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS photo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                file_name TEXT NOT NULL,
                parameters TEXT NOT NULL
            )",
            ()
        )?;

        Ok(())
    }

    fn create_thumbnail_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS thumbnail (
                photo_id INTEGER UNIQUE REFERENCES photo(id),
                data BLOB,
                width INTEGER,
                height INTEGER
            )",
            ()
        )?;

        Ok(())
    }
}