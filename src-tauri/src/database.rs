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

    Ok(())
}