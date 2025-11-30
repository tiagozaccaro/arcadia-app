// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;
mod models;
mod extensions;

use rusqlite::Connection;
use tauri::{AppHandle, Manager, State};
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::runtime::Runtime;
use crate::extensions::{ExtensionManager, StoreManager, fetch_store_extensions, fetch_extension_details, install_from_store, list_store_sources, add_store_source, remove_store_source, update_store_source};
use crate::models::ExtensionInfo;
use serde_json::Value;
use std::path::PathBuf;
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
async fn install_extension(app: AppHandle, manifest_path: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<String, String> {
    let mut manager = extension_manager.inner().write().await;
    let path = std::path::Path::new(&manifest_path);
    manager.load_extension(path).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn uninstall_extension(app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.unload_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn enable_extension(app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.enable_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn disable_extension(app: AppHandle, extension_id: String, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<(), String> {
    let mut manager = extension_manager.inner().write().await;
    manager.disable_extension(&extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn list_extensions(extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Vec<ExtensionInfo>, String> {
    let manager = extension_manager.inner().read().await;
    Ok(manager.list_extensions())
}

#[tauri::command]
async fn call_extension_api(app: AppHandle, extension_id: String, api: String, params: Value, extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Value, String> {
    let manager = extension_manager.inner().read().await;
    if let Some(extension) = manager.get_extension(&extension_id) {
        extension.handle_hook(&api, params).await.map_err(|e| e.to_string())
    } else {
        Err("Extension not found".to_string())
    }
}

#[tauri::command]
async fn get_extension_menu_items(extension_manager: State<'_, Arc<RwLock<ExtensionManager>>>) -> Result<Vec<crate::models::MenuItem>, String> {
    let manager = extension_manager.inner().read().await;
    let items = manager.get_extension_menu_items();
    println!("get_extension_menu_items: returning {} items", items.len());
    Ok(items)
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

            // Initialize extension manager
            let extension_dir = PathBuf::from("./extensions"); // Default extension directory
            let mut extension_manager = ExtensionManager::new(app.handle().clone(), extension_dir.clone());

            // Load sample extension on startup
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let sample_manifest = extension_dir.join("sample-game-library/manifest.json");
                if sample_manifest.exists() {
                    let _ = extension_manager.load_extension(&sample_manifest).await;
                }
            });

            app.manage(Arc::new(RwLock::new(extension_manager)));

            // Initialize store manager
            let store_manager = StoreManager::new();
            app.manage(Arc::new(RwLock::new(store_manager)));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet, get_setting, set_setting, get_app_data, save_app_data, update_app_data, delete_app_data, install_extension, uninstall_extension, enable_extension, disable_extension, list_extensions, call_extension_api, get_extension_menu_items, fetch_store_extensions, fetch_extension_details, install_from_store, list_store_sources, add_store_source, remove_store_source, update_store_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
