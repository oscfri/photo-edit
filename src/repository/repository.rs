use rusqlite::{Connection, Result};

pub struct Repository {
    connection: Connection
}

#[derive(Debug)]
struct Album {
    id: i32,
    name: String
}

#[derive(Debug)]
pub struct AlbumPhoto {
    id: i32,
    album_id: i32,
    pub file_name: String
}

impl Repository {
    pub fn new(connection: Connection) -> Self {
        Self {
            connection
        }
    }

    pub fn print_albums(&mut self) -> Result<()> {
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

    pub fn get_album_photos(&mut self, album_id: i32) -> Result<Vec<AlbumPhoto>> {
        let mut statement = self.connection.prepare(
            "SELECT id, album_id, file_name
                FROM album_photo
                WHERE album_id = ?1"
        )?;

        let rows = statement.query_map([album_id], |row| {
            Ok(AlbumPhoto {
                id: row.get(0)?,
                album_id: row.get(1)?,
                file_name: row.get(2)?
            })
        })?;
        
        Ok(rows.map(|row| row.unwrap()).collect())
    }
}