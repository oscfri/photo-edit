use rusqlite::{Connection, Result};

use super::repository::Repository;

pub struct RepositoryFactory {
    connection: Connection
}

impl<'a> RepositoryFactory {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn create(self) -> Repository {
        self.create_photo_table().unwrap();
        self.create_thumbnail_table().unwrap();
        Repository::new(self.connection)
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