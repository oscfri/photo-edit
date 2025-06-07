use std::sync::{Arc, Mutex};

use rusqlite::{Connection, Result};

use super::settings_repository::SettingsRepository;

pub struct SettingRepositoryFactory {
    connection: Arc<Mutex<Connection>>
}

impl SettingRepositoryFactory {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    pub fn create(self) -> SettingsRepository {
        self.create_settings_table().unwrap();
        SettingsRepository::new(self.connection)
    }

    fn create_settings_table(&self) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        connection.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                parameter TEXT PRIMARY KEY,
                value TEXT
            )",
            ()
        )?;

        Ok(())
    }
}