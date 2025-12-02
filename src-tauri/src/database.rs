use rusqlite::Connection;
use tauri::{App, Manager};
use chrono;
use crate::models::{Platform, Game};

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

    // Store sources table
    conn.execute(
        "CREATE TABLE IF NOT EXISTS store_sources (
              id TEXT PRIMARY KEY,
              name TEXT NOT NULL,
              source_type TEXT NOT NULL,
              base_url TEXT NOT NULL,
              enabled BOOLEAN DEFAULT 1,
              priority INTEGER DEFAULT 0,
              created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
              updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
          )",
        [],
    )?;

    // Game launcher tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS platforms (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL,
            description TEXT,
            icon_path TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS games (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            platform_id INTEGER NOT NULL,
            description TEXT,
            developer TEXT,
            publisher TEXT,
            release_date TEXT,
            cover_image_path TEXT,
            executable_path TEXT,
            working_directory TEXT,
            arguments TEXT,
            is_favorite BOOLEAN DEFAULT 0,
            playtime_minutes INTEGER DEFAULT 0,
            last_played DATETIME,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (platform_id) REFERENCES platforms(id) ON DELETE CASCADE
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS genres (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT UNIQUE NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS game_genres (
            game_id INTEGER NOT NULL,
            genre_id INTEGER NOT NULL,
            PRIMARY KEY (game_id, genre_id),
            FOREIGN KEY (game_id) REFERENCES games(id) ON DELETE CASCADE,
            FOREIGN KEY (genre_id) REFERENCES genres(id) ON DELETE CASCADE
        )",
        [],
    )?;

    Ok(())
}

// Platform CRUD functions
pub fn create_platform(conn: &Connection, name: String, description: Option<String>, icon_path: Option<String>) -> Result<i64, rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO platforms (name, description, icon_path, created_at, updated_at) VALUES (?, ?, ?, ?, ?)",
        rusqlite::params![name, description, icon_path, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_platforms(conn: &Connection) -> Result<Vec<Platform>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, description, icon_path, created_at, updated_at FROM platforms")?;
    let rows = stmt.query_map([], |row| {
        Ok(Platform {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            icon_path: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    })?;
    let mut platforms = Vec::new();
    for row in rows {
        platforms.push(row?);
    }
    Ok(platforms)
}

pub fn update_platform(conn: &Connection, id: i64, name: String, description: Option<String>, icon_path: Option<String>) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE platforms SET name = ?, description = ?, icon_path = ?, updated_at = ? WHERE id = ?",
        rusqlite::params![name, description, icon_path, now, id],
    )?;
    Ok(())
}

pub fn delete_platform(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM platforms WHERE id = ?", &[&id])?;
    Ok(())
}

// Game CRUD functions
pub fn create_game(
    conn: &Connection,
    name: String,
    platform_id: i64,
    description: Option<String>,
    developer: Option<String>,
    publisher: Option<String>,
    release_date: Option<String>,
    cover_image_path: Option<String>,
    executable_path: Option<String>,
    working_directory: Option<String>,
    arguments: Option<String>,
) -> Result<i64, rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO games (name, platform_id, description, developer, publisher, release_date, cover_image_path, executable_path, working_directory, arguments, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        rusqlite::params![name, platform_id, description, developer, publisher, release_date, cover_image_path, executable_path, working_directory, arguments, now, now],
    )?;
    Ok(conn.last_insert_rowid())
}

pub fn get_games(conn: &Connection) -> Result<Vec<Game>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, platform_id, description, developer, publisher, release_date, cover_image_path, executable_path, working_directory, arguments, is_favorite, playtime_minutes, last_played, created_at, updated_at FROM games")?;
    let rows = stmt.query_map([], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            platform_id: row.get(2)?,
            description: row.get(3)?,
            developer: row.get(4)?,
            publisher: row.get(5)?,
            release_date: row.get(6)?,
            cover_image_path: row.get(7)?,
            executable_path: row.get(8)?,
            working_directory: row.get(9)?,
            arguments: row.get(10)?,
            is_favorite: row.get(11)?,
            playtime_minutes: row.get(12)?,
            last_played: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    })?;
    let mut games = Vec::new();
    for row in rows {
        games.push(row?);
    }
    Ok(games)
}

pub fn get_games_by_platform(conn: &Connection, platform_id: i64) -> Result<Vec<Game>, rusqlite::Error> {
    let mut stmt = conn.prepare("SELECT id, name, platform_id, description, developer, publisher, release_date, cover_image_path, executable_path, working_directory, arguments, is_favorite, playtime_minutes, last_played, created_at, updated_at FROM games WHERE platform_id = ?")?;
    let rows = stmt.query_map([platform_id], |row| {
        Ok(Game {
            id: row.get(0)?,
            name: row.get(1)?,
            platform_id: row.get(2)?,
            description: row.get(3)?,
            developer: row.get(4)?,
            publisher: row.get(5)?,
            release_date: row.get(6)?,
            cover_image_path: row.get(7)?,
            executable_path: row.get(8)?,
            working_directory: row.get(9)?,
            arguments: row.get(10)?,
            is_favorite: row.get(11)?,
            playtime_minutes: row.get(12)?,
            last_played: row.get(13)?,
            created_at: row.get(14)?,
            updated_at: row.get(15)?,
        })
    })?;
    let mut games = Vec::new();
    for row in rows {
        games.push(row?);
    }
    Ok(games)
}

pub fn update_game(
    conn: &Connection,
    id: i64,
    name: String,
    platform_id: i64,
    description: Option<String>,
    developer: Option<String>,
    publisher: Option<String>,
    release_date: Option<String>,
    cover_image_path: Option<String>,
    executable_path: Option<String>,
    working_directory: Option<String>,
    arguments: Option<String>,
) -> Result<(), rusqlite::Error> {
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "UPDATE games SET name = ?, platform_id = ?, description = ?, developer = ?, publisher = ?, release_date = ?, cover_image_path = ?, executable_path = ?, working_directory = ?, arguments = ?, updated_at = ? WHERE id = ?",
        rusqlite::params![name, platform_id, description, developer, publisher, release_date, cover_image_path, executable_path, working_directory, arguments, now, id],
    )?;
    Ok(())
}

pub fn delete_game(conn: &Connection, id: i64) -> Result<(), rusqlite::Error> {
    conn.execute("DELETE FROM games WHERE id = ?", &[&id])?;
    Ok(())
}