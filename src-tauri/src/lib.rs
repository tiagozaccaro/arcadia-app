// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;

use rusqlite::Connection;
use tauri::{AppHandle, Manager};
#[tauri::command]
fn get_setting(app: AppHandle, key: String) -> Result<String, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?").map_err(|e| e.to_string())?;
    let value: String = stmt.query_row([key], |row| row.get(0)).map_err(|e| e.to_string())?;
    Ok(value)
}

#[tauri::command]
fn set_setting(app: AppHandle, key: String, value: String) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("INSERT OR REPLACE INTO settings (key, value) VALUES (?, ?)", [key, value]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn get_app_data(app: AppHandle, data_type: String) -> Result<Vec<String>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT data FROM app_data WHERE data_type = ?").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([data_type], |row| row.get::<_, String>(0)).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
fn save_app_data(app: AppHandle, data_type: String, data: String) -> Result<i64, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("INSERT INTO app_data (data_type, data) VALUES (?, ?)", [data_type, data]).map_err(|e| e.to_string())?;
    let id = conn.last_insert_rowid();
    Ok(id)
}

#[tauri::command]
fn update_app_data(app: AppHandle, id: i64, data: String) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let affected = conn.execute("UPDATE app_data SET data = ? WHERE id = ?", [data, id.to_string()]).map_err(|e| e.to_string())?;
    if affected == 0 {
        return Err("No row updated".to_string());
    }
    Ok(())
}

#[tauri::command]
fn delete_app_data(app: AppHandle, id: i64) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let affected = conn.execute("DELETE FROM app_data WHERE id = ?", [id]).map_err(|e| e.to_string())?;
    if affected == 0 {
        return Err("No row deleted".to_string());
    }
    Ok(())
}
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            database::init_database(app).expect("Failed to init database");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_setting, set_setting, get_app_data, save_app_data, update_app_data, delete_app_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
