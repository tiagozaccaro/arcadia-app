use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExtensionType {
    Theme,
    DataSource,
    GameLibrary,
}

impl From<String> for ExtensionType {
    fn from(s: String) -> Self {
        match s.as_str() {
            "theme" => ExtensionType::Theme,
            "data_source" => ExtensionType::DataSource,
            "game_library" => ExtensionType::GameLibrary,
            _ => ExtensionType::Theme, // default
        }
    }
}

impl ToString for ExtensionType {
    fn to_string(&self) -> String {
        match self {
            ExtensionType::Theme => "theme".to_string(),
            ExtensionType::DataSource => "data_source".to_string(),
            ExtensionType::GameLibrary => "game_library".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionManifest {
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub extension_type: ExtensionType,
    pub entry_point: String,
    pub permissions: Vec<String>,
    pub dependencies: Option<HashMap<String, String>>,
    pub hooks: Option<Vec<String>>,
    pub apis: Option<ExtensionApis>,
    #[serde(rename = "menuItems")]
    pub menu_items: Option<Vec<MenuItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuItem {
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub items: Option<Vec<MenuSubItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuSubItem {
    pub title: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionApis {
    pub provided: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct Extension {
    pub id: String,
    pub manifest: ExtensionManifest,
    pub path: std::path::PathBuf,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: Option<String>,
    pub description: Option<String>,
    pub extension_type: String,
    pub enabled: bool,
}

#[derive(Debug, Clone)]
pub struct ExtensionPermission {
    pub extension_id: String,
    pub permission: String,
    pub granted: bool,
}

#[derive(Debug, Clone)]
pub struct ExtensionSetting {
    pub extension_id: String,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug)]
pub enum ExtensionError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Database(rusqlite::Error),
    Validation(String),
    NotFound(String),
    PermissionDenied(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::Io(e) => write!(f, "IO error: {}", e),
            ExtensionError::Json(e) => write!(f, "JSON error: {}", e),
            ExtensionError::Database(e) => write!(f, "Database error: {}", e),
            ExtensionError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ExtensionError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ExtensionError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for ExtensionError {}

impl From<std::io::Error> for ExtensionError {
    fn from(e: std::io::Error) -> Self {
        ExtensionError::Io(e)
    }
}

impl From<serde_json::Error> for ExtensionError {
    fn from(e: serde_json::Error) -> Self {
        ExtensionError::Json(e)
    }
}

impl From<rusqlite::Error> for ExtensionError {
    fn from(e: rusqlite::Error) -> Self {
        ExtensionError::Database(e)
    }
}