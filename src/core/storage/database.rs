use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::fs;

pub struct Database {
    path: PathBuf,
}

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
        let home_dir = dirs::home_dir().ok_or(rusqlite::Error::InvalidPath(PathBuf::from("Home directory not found")))?;
        let app_dir = home_dir.join(".rostr");
        
        if !app_dir.exists() {
            fs::create_dir_all(&app_dir).map_err(|_| rusqlite::Error::InvalidPath(app_dir.clone()))?;
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

        conn.execute("CREATE INDEX IF NOT EXISTS idx_employees_name ON employees(name)", [])?;
        conn.execute("CREATE INDEX IF NOT EXISTS idx_schedules_year_month ON schedules(year, month)", [])?;

        Ok(())
    }

    pub fn clear_all_data(&self) -> Result<()> {
        let mut conn = self.get_connection()?;
        let tx = conn.transaction()?;
        tx.execute("DELETE FROM schedules", [])?;
        tx.execute("DELETE FROM employees", [])?;
        tx.commit()?;
        Ok(())
    }
}
