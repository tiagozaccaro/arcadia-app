// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;
mod models;
mod extensions;

use crate::database::{create_platform, get_platforms, update_platform, delete_platform, create_game, get_games, get_games_by_platform, update_game, delete_game};
use arcadia_extension_framework::store::models::StoreSource;

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};
use std::sync::Arc;
use tokio::sync::RwLock;
use arcadia_extension_framework::models::{ExtensionInfo, MenuItem};
use arcadia_extension_framework::store::manager::StoreManager;
use crate::extensions::{ExtensionManager, fetch_store_extensions, fetch_extension_details, install_from_store, list_store_sources, add_store_source, remove_store_source, update_store_source};
use serde_json::Value;
use std::path::PathBuf;
#[tauri::command]
fn get_setting(app: AppHandle, key: String) -> Result<String, String> {
    println!("get_setting called with key: {}", key);
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = ?").map_err(|e| e.to_string())?;
    let value: String = stmt.query_row([key], |row| row.get(0)).map_err(|e| e.to_string())?;
    println!("get_setting returning: {}", value);
    Ok(value)
}

#[tauri::command]
fn set_setting(app: AppHandle, key: String, value: String) -> Result<(), String> {
    println!("set_setting called with key: {}, value: {}", key, value);
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
fn get_extension_setting(app: AppHandle, extension_id: String, key: String) -> Result<String, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT value FROM extension_settings WHERE extension_id = ? AND key = ?").map_err(|e| e.to_string())?;
    let value: String = stmt.query_row([extension_id, key], |row| row.get(0)).map_err(|e| e.to_string())?;
    Ok(value)
}

#[tauri::command]
fn set_extension_setting(app: AppHandle, extension_id: String, key: String, value: String) -> Result<(), String> {
    println!("set_extension_setting called with extension_id: {}, key: {}, value: {}", extension_id, key, value);
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    conn.execute("INSERT OR REPLACE INTO extension_settings (extension_id, key, value) VALUES (?, ?, ?)", [extension_id, key, value]).map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn list_extension_settings(app: AppHandle, extension_id: String) -> Result<Vec<(String, String)>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT key, value FROM extension_settings WHERE extension_id = ?").map_err(|e| e.to_string())?;
    let rows = stmt.query_map([extension_id], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, String>(1)?))
    }).map_err(|e| e.to_string())?;
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    Ok(result)
}

#[tauri::command]
fn delete_extension_setting(app: AppHandle, extension_id: String, key: String) -> Result<(), String> {
    println!("delete_extension_setting called with extension_id: {}, key: {}", extension_id, key);
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    let affected = conn.execute("DELETE FROM extension_settings WHERE extension_id = ? AND key = ?", [extension_id, key]).map_err(|e| e.to_string())?;
    println!("delete_extension_setting affected {} rows", affected);
    if affected == 0 {
        return Err("No row deleted".to_string());
    }
    Ok(())
}

#[tauri::command]
async fn install_extension(_app: AppHandle, manifest_path: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<String, String> {
    let mut manager = extension_manager.inner().write().await;
    let path = std::path::Path::new(&manifest_path);
    manager.load_extension(path).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn uninstall_extension(_app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.unload_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn enable_extension(_app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.enable_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn disable_extension(_app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.disable_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_extensions(extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Vec<ExtensionInfo>, String> {
    let manager = extension_manager.inner().read().await;
    Ok(manager.list_extensions())
}

#[tauri::command]
async fn call_extension_api(_app: AppHandle, extension_id: String, api: String, params: Value, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Value, String> {
    let manager = extension_manager.inner().read().await;
    if let Some(extension) = manager.get_extension(&extension_id) {
        extension.handle_hook(&api, params).await.map_err(|e| e.to_string())
    } else {
        Err("Extension not found".to_string())
    }
}

#[tauri::command]
async fn get_extension_menu_items(extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Vec<MenuItem>, String> {
    let manager = extension_manager.inner().read().await;
    let items = manager.get_extension_menu_items();
    println!("get_extension_menu_items: returning {} items", items.len());
    Ok(items)
}

// Platform commands
#[tauri::command]
fn create_platform_command(app: AppHandle, name: String, description: Option<String>, icon_path: Option<String>) -> Result<i64, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    create_platform(&conn, name, description, icon_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_platforms_command(app: AppHandle) -> Result<Vec<crate::models::Platform>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    get_platforms(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_platform_command(app: AppHandle, id: i64, name: String, description: Option<String>, icon_path: Option<String>) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    update_platform(&conn, id, name, description, icon_path).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_platform_command(app: AppHandle, id: i64) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    delete_platform(&conn, id).map_err(|e| e.to_string())
}

// Game commands
use crate::database::GameData;

#[tauri::command]
fn create_game_command(
    app: AppHandle,
    game_data: GameData,
) -> Result<i64, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    create_game(&conn, game_data).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_games_command(app: AppHandle) -> Result<Vec<crate::models::Game>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    get_games(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
fn get_games_by_platform_command(app: AppHandle, platform_id: i64) -> Result<Vec<crate::models::Game>, String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    get_games_by_platform(&conn, platform_id).map_err(|e| e.to_string())
}

#[tauri::command]
fn update_game_command(
    app: AppHandle,
    id: i64,
    game_data: GameData,
) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    update_game(&conn, id, game_data).map_err(|e| e.to_string())
}

#[tauri::command]
fn delete_game_command(app: AppHandle, id: i64) -> Result<(), String> {
    let data_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = data_dir.join("app.db");
    let conn = Connection::open(db_path).map_err(|e| e.to_string())?;
    delete_game(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    println!("Tauri app starting in debug mode");
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            println!("Setting up app");
            database::init_database(app).expect("Failed to init database");

            // Initialize extension manager
            let extension_dir = PathBuf::from("./extensions"); // Default extension directory
            let extension_manager = ExtensionManager::new(app.handle().clone(), extension_dir.clone());


            app.manage(Arc::new(RwLock::new(extension_manager)));

            // Initialize store manager
            let mut store_manager = StoreManager::new();

            // Rename default source to "Arcadia Store" and update URL if it exists
            let sources = store_manager.list_sources();
            println!("Found {} sources during initialization", sources.len());
            for source in sources {
                println!("Source: {} - {} - {}", source.id, source.name, source.base_url);
                // Update any source that looks like a default/local store
                let updated_source = StoreSource {
                    id: source.id.clone(),
                    name: "Arcadia Store".to_string(),
                    source_type: source.source_type,
                    base_url: "https://raw.githubusercontent.com/tiagozaccaro/arcadia-app/main/arcadia-store/store-manifest.json".to_string(),
                    enabled: true, // Make sure it's enabled
                    priority: source.priority,
                };
                match store_manager.update_source(updated_source) {
                    Ok(_) => println!("Successfully updated source {}", source.id),
                    Err(e) => println!("Failed to update source {}: {:?}", source.id, e),
                }
            }

            app.manage(Arc::new(RwLock::new(store_manager)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_setting, set_setting, get_app_data, save_app_data, update_app_data, delete_app_data, get_extension_setting, set_extension_setting, list_extension_settings, delete_extension_setting, install_extension, uninstall_extension, enable_extension, disable_extension, list_extensions, call_extension_api, get_extension_menu_items, fetch_store_extensions, fetch_extension_details, install_from_store, list_store_sources, add_store_source, remove_store_source, update_store_source, create_platform_command, get_platforms_command, update_platform_command, delete_platform_command, create_game_command, get_games_command, get_games_by_platform_command, update_game_command, delete_game_command])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        let result = greet("World");
        assert_eq!(result, "Hello, World! You've been greeted from Rust!");
    }
}
