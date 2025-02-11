use rusqlite::{Connection, Result};

use super::repository::Repository;

pub struct RepositoryFactory {
    // TODO: Should include connection configuration here
}

impl<'a> RepositoryFactory {
    pub fn new() -> Self {
        Self {}
    }

    pub fn create(&self) -> Result<Repository> {
        let connection: Connection = Connection::open_in_memory()?;

        connection.execute(
            "CREATE TABLE album (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL
            )",
            ()
        )?;

        connection.execute(
            "CREATE TABLE album_photo (
                id INTEGER PRIMARY KEY,
                album_id INTEGER,
                file_name TEXT NOT NULL,
                FOREIGN KEY(album_id) REFERENCES album(id)
            )",
            ()
        )?;
        
        connection.execute("INSERT INTO album (id, name) VALUES (?1, ?2)", (0, &"First album"))?;
        connection.execute(
            "INSERT INTO album_photo (id, album_id, file_name)
                VALUES (?1, ?2, ?3)",
            (0, 0, &"example.png")
        )?;
        connection.execute(
            "INSERT INTO album_photo (id, album_id, file_name)
                VALUES (?1, ?2, ?3)",
            (1, 0, &"example2.jpg")
        )?;

        Ok(Repository::new(connection))
    }
}