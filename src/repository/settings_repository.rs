use std::sync::{Arc, Mutex};

use rusqlite::{Connection, OptionalExtension, Result};

use super::parameter_name::ParameterName;

pub struct SettingsRepository {
    connection: Arc<Mutex<Connection>>
}

impl SettingsRepository {
    pub fn new(connection: Arc<Mutex<Connection>>) -> Self {
        Self { connection }
    }

    pub fn get_parameter_value(&self, parameter: ParameterName) -> Result<Option<String>> {
        let connection = self.connection.lock().unwrap();
        let mut statement = connection.prepare(
            "SELECT value
                FROM settings
                WHERE parameter = ?1
                LIMIT 1"
        )?;

        statement.query_row([parameter.to_string()], |row| { row.get(0) }).optional()
    }

    pub fn set_parameter_value(&self, parameter: ParameterName, value: &String) -> Result<()> {
        let connection = self.connection.lock().unwrap();
        connection.execute(
            "INSERT OR REPLACE INTO settings (parameter, value)
                VALUES (?1, ?2)",
            [&parameter.to_string(), value])?;
        
        Ok(())
    }
}