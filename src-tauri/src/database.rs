use rusqlite::Connection;
use tauri::{App, Manager};

pub fn init_database(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let data_dir = app.path().app_data_dir()?;
    let db_path = data_dir.join("app.db");
    std::fs::create_dir_all(&data_dir)?;
 
    let conn = Connection::open(db_path)?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY,
            key TEXT UNIQUE,
            value TEXT
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_data (
             id INTEGER PRIMARY KEY,
             data_type TEXT,
             data TEXT
         )",
        [],
    )?;

    // Extension system tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS extensions (
             id TEXT PRIMARY KEY,
             name TEXT NOT NULL,
             version TEXT NOT NULL,
             author TEXT,
             description TEXT,
             type TEXT NOT NULL,
             entry_point TEXT NOT NULL,
             manifest_path TEXT NOT NULL,
             enabled BOOLEAN DEFAULT 1,
             installed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
             updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS extension_permissions (
             id INTEGER PRIMARY KEY,
             extension_id TEXT,
             permission TEXT NOT NULL,
             granted BOOLEAN DEFAULT 0,
             FOREIGN KEY (extension_id) REFERENCES extensions(id)
         )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS extension_settings (
             id INTEGER PRIMARY KEY,
             extension_id TEXT,
             key TEXT NOT NULL,
             value TEXT,
             FOREIGN KEY (extension_id) REFERENCES extensions(id)
         )",
        [],
    )?;

    Ok(())
}