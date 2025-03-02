use std::path::PathBuf;

use rusqlite::{Connection, Result};

pub struct Repository {
    connection: Connection
}

#[derive(Debug)]
struct Album {
    id: i32,
    name: String
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
        Self {
            connection
        }
    }

    pub fn print_albums(&self) -> Result<()> {
        let mut statement = self.connection.prepare("SELECT id, name FROM album")?;
        let albums = statement.query_map([], |row| {
            Ok(Album {
                id: row.get(0)?,
                name: row.get(1)?
            })
        })?;

        for album in albums {
            println!("Found album {:?}", album.unwrap());
        }

        Ok(())
    }

    pub fn get_album_photos(&self, album_id: i32) -> Result<Vec<AlbumPhoto>> {
        let mut statement = self.connection.prepare(
            "SELECT id, file_name, parameters
                FROM photo
                WHERE album_id = ?1"
        )?;

        let rows = statement.query_map([album_id], |row| {
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
            "INSERT INTO photo (album_id, file_name, parameters)
                VALUES (?1, ?2, ?3)",
            (0, &path.to_str(), &"{}")
        )?;

        Ok(())
    }
}