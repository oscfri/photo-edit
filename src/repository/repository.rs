use std::{io::Write, path::PathBuf};

use rusqlite::{Connection, Result};

use crate::types::RawImage;

pub struct Repository {
    connection: Connection
}

// TODO: Need to come up with a good naming convention for this...
pub struct AlbumPhotoDto {
    pub id: i32,
    pub file_name: String,
    pub parameters: String,
    pub thumbnail: Option<RawImage>
}

impl Repository {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn get_album_photos(&self) -> Result<Vec<AlbumPhotoDto>> {
        let mut statement = self.connection.prepare(
            "SELECT
                    photo.id,
                    photo.file_name,
                    photo.parameters,
                    thumbnail.data,
                    thumbnail.width,
                    thumbnail.height
                FROM photo
                LEFT OUTER JOIN thumbnail
                ON photo.id = thumbnail.photo_id"
        )?;

        let rows = statement.query_map([], |row| {
            let thumbnail_data: Option<Vec<u8>> = row.get(3)?;
            let thumbnail_width: Option<usize> = row.get(4)?;
            let thumbnail_height: Option<usize> = row.get(4)?;

            let thumbnail = match (thumbnail_data, thumbnail_width, thumbnail_height) {
                (Some(data), Some(width), Some(height)) => {
                    Some(RawImage {
                        pixels: data,
                        width: width,
                        height: height
                    })
                },
                _ => None
            };
            
            Ok(AlbumPhotoDto {
                id: row.get(0)?,
                file_name: row.get(1)?,
                parameters: row.get(2)?,
                thumbnail: thumbnail
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

    pub fn add_thumbnail(&self, photo_id: i32, raw_image: &RawImage) -> Result<()> {
        let pixels = &raw_image.pixels;
        let width = raw_image.width as i32;
        let height = raw_image.height as i32;

        self.connection.execute(
            "INSERT OR REPLACE INTO thumbnail (photo_id, data, width, height)
                VALUES (?1, ZEROBLOB(?2), ?3, ?4)",
            [&photo_id, &(pixels.len() as i32), &width, &height]
        )?;

        let row_id = self.connection.last_insert_rowid();
        let mut blob = self.connection.blob_open(
            rusqlite::DatabaseName::Main,
            "thumbnail",
            "data",
            row_id,
            false
        )?;

        blob.write(pixels).unwrap();

        Ok(())
    }

    pub fn delete_photo(&self, photo_id: i32) -> Result<()> {
        self.connection.execute(
            "DELETE FROM thumbnail
            WHERE photo_id = ?1",
            [photo_id]
        )?;
        self.connection.execute(
            "DELETE FROM photo
            WHERE id = ?1",
            [photo_id]
        )?;

        Ok(())
    }
}