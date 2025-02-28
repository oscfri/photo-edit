use rusqlite::{Connection, Result};

use super::repository::Repository;

pub struct RepositoryFactory {
    connection: Connection
}

impl<'a> RepositoryFactory {
    pub fn new(connection: Connection) -> Self {
        Self { connection }
    }

    pub fn create(self) -> Result<Repository> {
        self.create_album_table()?;
        self.create_photo_table()?;
        
        // self.connection.execute("INSERT INTO album (id, name) VALUES (?1, ?2)", (0, &"First album"))?;
        // self.connection.execute(
        //     "INSERT INTO photo (id, album_id, file_name, parameters)
        //         VALUES (?1, ?2, ?3, ?4)",
        //     (0, 0, &"example.png", &"{}")
        // )?;
        // self.connection.execute(
        //     "INSERT INTO photo (id, album_id, file_name, parameters)
        //         VALUES (?1, ?2, ?3, ?4)",
        //     (1, 0, &"example2.jpg", &"{}")
        // )?;

        Ok(Repository::new(self.connection))
    }

    fn create_album_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS album (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL
            )",
            ()
        )?;

        Ok(())
    }

    fn create_photo_table(&self) -> Result<()> {
        self.connection.execute(
            "CREATE TABLE IF NOT EXISTS photo (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                album_id INTEGER,
                file_name TEXT NOT NULL,
                parameters TEXT NOT NULL,
                FOREIGN KEY(album_id) REFERENCES album(id)
            )",
            ()
        )?;

        Ok(())
    }
}