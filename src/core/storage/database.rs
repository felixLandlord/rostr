use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use rusqlite::{Connection, Result};
use std::fs;
use std::path::PathBuf;

pub struct Database {
    path: PathBuf,
}

const ENCRYPTION_KEY: &[u8; 32] = b"rostr_secure_key_32_bytes_long_!";

impl Database {
    pub fn new() -> Result<Self> {
        let path = Self::get_database_path()?;
        let db = Self { path };
        db.initialize()?;
        Ok(db)
    }

    pub fn get_connection(&self) -> Result<Connection> {
        Connection::open(&self.path)
    }

    fn get_database_path() -> Result<PathBuf> {
        let home_dir = dirs::home_dir().ok_or(rusqlite::Error::InvalidPath(PathBuf::from(
            "Home directory not found",
        )))?;
        let app_dir = home_dir.join(".rostr");

        if !app_dir.exists() {
            fs::create_dir_all(&app_dir)
                .map_err(|_| rusqlite::Error::InvalidPath(app_dir.clone()))?;
        }

        Ok(app_dir.join("rostr.db"))
    }

    fn initialize(&self) -> Result<()> {
        let conn = self.get_connection()?;

        conn.execute("PRAGMA foreign_keys = ON;", [])?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS employees (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                sex TEXT NOT NULL,
                role TEXT NOT NULL,
                required_days INTEGER NOT NULL,
                fixed_days TEXT,
                is_mentor INTEGER NOT NULL DEFAULT 0,
                is_mentee INTEGER NOT NULL DEFAULT 0,
                mentor_id INTEGER,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                deleted_at DATETIME,
                FOREIGN KEY (mentor_id) REFERENCES employees(id) ON DELETE SET NULL
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS schedules (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                year INTEGER NOT NULL,
                month INTEGER NOT NULL,
                schedule_data TEXT NOT NULL,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                UNIQUE(year, month)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;

        // Migration: Add deleted_at column if it doesn't exist (for existing DBs)
        let _ = conn.execute("ALTER TABLE employees ADD COLUMN deleted_at DATETIME", []);

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_employees_name ON employees(name)",
            [],
        )?;
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_schedules_year_month ON schedules(year, month)",
            [],
        )?;

        Ok(())
    }

    pub fn clear_all_data(&self) -> Result<()> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM schedules", [])?;
        tx.execute("DELETE FROM employees", [])?;
        tx.execute("DELETE FROM settings WHERE key = 'api_key'", [])?;
        tx.commit()?;
        Ok(())
    }

    fn encrypt(&self, plaintext: &str) -> Result<String, String> {
        let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        let nonce_bytes: [u8; 12] = rand::random();
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| format!("Encryption failed: {}", e))?;
        let mut combined = nonce_bytes.to_vec();
        combined.extend(ciphertext);
        Ok(BASE64.encode(&combined))
    }

    fn decrypt(&self, encrypted: &str) -> Result<String, String> {
        let combined = BASE64
            .decode(encrypted)
            .map_err(|e| format!("Base64 decode failed: {}", e))?;
        if combined.len() < 12 {
            return Err("Invalid encrypted data".to_string());
        }
        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let cipher = Aes256Gcm::new_from_slice(ENCRYPTION_KEY)
            .map_err(|e| format!("Failed to create cipher: {}", e))?;
        let nonce = Nonce::from_slice(nonce_bytes);
        let plaintext = cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| format!("Decryption failed: {}", e))?;
        String::from_utf8(plaintext).map_err(|e| format!("UTF-8 decode failed: {}", e))
    }

    pub fn save_api_key(&self, api_key: &str) -> Result<(), String> {
        let conn = self.get_connection().map_err(|e| e.to_string())?;
        let encrypted = self.encrypt(api_key)?;
        conn.execute(
            "INSERT OR REPLACE INTO settings (key, value) VALUES ('api_key', ?1)",
            rusqlite::params![encrypted],
        )
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn get_api_key(&self) -> Result<Option<String>, String> {
        let conn = self.get_connection().map_err(|e| e.to_string())?;
        let result: std::result::Result<String, _> = conn.query_row(
            "SELECT value FROM settings WHERE key = 'api_key'",
            [],
            |row| row.get(0),
        );
        match result {
            Ok(encrypted) => Ok(Some(self.decrypt(&encrypted)?)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn delete_api_key(&self) -> Result<(), String> {
        let conn = self.get_connection().map_err(|e| e.to_string())?;
        conn.execute("DELETE FROM settings WHERE key = 'api_key'", [])
            .map_err(|e| e.to_string())?;
        Ok(())
    }
}
