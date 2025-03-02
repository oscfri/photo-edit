use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub struct Repository {
    connection: Connection
}

// TODO: Need to come up with a good naming convention for this...
#[derive(Debug)]
pub struct AlbumPhoto {
    pub id: i32,
    pub file_name: String,
    pub parameters: String
}

impl Repository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn get_album_photos(&self) -> Result<Vec<AlbumPhoto>> {
        let mut statement = self.connection.prepare(
            "SELECT id, file_name, parameters
                FROM photo"
        )?;

        let rows = statement.query_map([], |row| {
            Ok(AlbumPhoto {
                id: row.get(0)?,
                file_name: row.get(1)?,
                parameters: row.get(2)?,
            })
        })?;
        
        Ok(rows.map(|row| row.unwrap()).collect())
    }

    pub fn save_photo_parameters(&self, photo_id: i32, parameters: String) -> Result<()> {
        self.connection.execute(
            "UPDATE photo
                SET parameters = ?2
                WHERE id = ?1",
            (photo_id, &parameters)
        )?;

        Ok(())
    }

    pub fn add_photo(&self, path: &PathBuf) -> Result<()> {
        self.connection.execute(
            "INSERT INTO photo (file_name, parameters)
                VALUES (?1, ?2)",
            (&path.to_str(), &"{}")
        )?;

        Ok(())
    }

    pub fn delete_photo(&self, photo_id: i32) -> Result<()> {
        self.connection.execute(
            "DELETE FROM photo
            WHERE id = ?1",
            [photo_id]
        )?;

        Ok(())
    }
}