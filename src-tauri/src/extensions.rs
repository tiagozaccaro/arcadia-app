use crate::models::*;
use async_trait::async_trait;
use reqwest::Client;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use uuid::Uuid;
use urlencoding;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum StoreSourceType {
    Official,
    Community,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreSource {
    pub id: String,
    pub name: String,
    pub source_type: StoreSourceType,
    pub base_url: String,
    pub enabled: bool,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreExtension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extension_type: ExtensionType,
    pub download_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreExtensionDetails {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extension_type: ExtensionType,
    pub download_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
    pub manifest_url: String,
    pub package_url: String,
    pub checksum: String,
    pub readme: String,
    pub screenshots: Vec<String>,
    pub dependencies: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreFilters {
    pub extension_type: Option<ExtensionType>,
    pub tags: Option<Vec<String>>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortOption {
    Name,
    DownloadCount,
    Rating,
    Newest,
}

#[derive(Debug)]
pub enum StoreError {
    Network(reqwest::Error),
    Json(serde_json::Error),
    Validation(String),
    Security(String),
}

impl std::fmt::Display for StoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StoreError::Network(e) => write!(f, "Network error: {}", e),
            StoreError::Json(e) => write!(f, "JSON error: {}", e),
            StoreError::Validation(msg) => write!(f, "Validation error: {}", msg),
            StoreError::Security(msg) => write!(f, "Security error: {}", msg),
        }
    }
}

impl std::error::Error for StoreError {}

impl From<reqwest::Error> for StoreError {
    fn from(e: reqwest::Error) -> Self {
        StoreError::Network(e)
    }
}

impl From<serde_json::Error> for StoreError {
    fn from(e: serde_json::Error) -> Self {
        StoreError::Json(e)
    }
}

#[async_trait]
pub trait ExtensionImpl: Send + Sync {
    async fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError>;
    async fn shutdown(&mut self) -> Result<(), ExtensionError>;
    async fn handle_hook(&self, hook: &str, params: Value) -> Result<Value, ExtensionError>;
    fn get_manifest(&self) -> &ExtensionManifest;
    fn get_type(&self) -> ExtensionType;
    fn get_id(&self) -> &str;
}

pub struct ExtensionContext {
    pub app_handle: AppHandle,
    pub extension_dir: PathBuf,
}

pub struct ExtensionManager {
    extensions: HashMap<String, Box<dyn ExtensionImpl>>,
    registry: ExtensionRegistry,
    context: ExtensionContext,
}

impl ExtensionManager {
    pub fn new(app_handle: AppHandle, extension_dir: PathBuf) -> Self {
        Self {
            extensions: HashMap::new(),
            registry: ExtensionRegistry::new(),
            context: ExtensionContext {
                app_handle,
                extension_dir,
            },
        }
    }

    pub async fn load_extension(&mut self, manifest_path: &Path) -> Result<String, ExtensionError> {
        // Parse manifest
        let manifest = self.parse_manifest(manifest_path)?;

        // Validate manifest
        self.validate_manifest(&manifest)?;

        // Generate unique ID
        let id = Uuid::new_v4().to_string();

        // Create extension instance (stub for now - would load actual extension code)
        let mut extension = self.create_extension(&id, manifest, manifest_path.parent().unwrap().to_path_buf())?;

        // Initialize extension
        extension.initialize(&self.context).await?;

        // Store in database
        self.save_extension_to_db(&id, &extension.get_manifest(), manifest_path).await?;

        // Register permissions
        self.save_permissions_to_db(&id, &extension.get_manifest().permissions).await?;

        // Add to registry
        self.registry.register(ExtensionInfo {
            id: id.clone(),
            name: extension.get_manifest().name.clone(),
            version: extension.get_manifest().version.clone(),
            author: extension.get_manifest().author.clone(),
            description: extension.get_manifest().description.clone(),
            extension_type: extension.get_type().to_string(),
            enabled: true,
        });

        // Store extension
        self.extensions.insert(id.clone(), extension);

        Ok(id)
    }

    pub async fn unload_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        if let Some(mut extension) = self.extensions.remove(id) {
            extension.shutdown().await?;
            self.registry.unregister(id);
            self.remove_extension_from_db(id).await?;
        }
        Ok(())
    }

    pub async fn call_hook(&self, hook: &str, params: Value) -> Result<Vec<Value>, ExtensionError> {
        let mut results = Vec::new();
        for extension in self.extensions.values() {
            if let Ok(result) = extension.handle_hook(hook, params.clone()).await {
                results.push(result);
            }
        }
        Ok(results)
    }

    pub fn get_extension(&self, id: &str) -> Option<&Box<dyn ExtensionImpl>> {
        self.extensions.get(id)
    }

    pub fn list_extensions(&self) -> Vec<ExtensionInfo> {
        self.registry.get_all()
    }

    pub fn get_extension_menu_items(&self) -> Vec<MenuItem> {
        let mut all_menu_items = Vec::new();
        let enabled_ids: std::collections::HashSet<String> = self.registry.get_enabled().into_iter().map(|e| e.id).collect();
        for extension in self.extensions.values() {
            if enabled_ids.contains(extension.get_id()) {
                if let Some(menu_items) = &extension.get_manifest().menu_items {
                    all_menu_items.extend(menu_items.clone());
                }
            }
        }
        all_menu_items
    }

    pub async fn enable_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        if let Some(extension_info) = self.registry.extensions.get_mut(id) {
            extension_info.enabled = true;
            self.update_extension_enabled_in_db(id, true).await?;
            Ok(())
        } else {
            Err(ExtensionError::NotFound(format!("Extension {} not found", id)))
        }
    }

    pub async fn disable_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        if let Some(extension_info) = self.registry.extensions.get_mut(id) {
            extension_info.enabled = false;
            self.update_extension_enabled_in_db(id, false).await?;
            Ok(())
        } else {
            Err(ExtensionError::NotFound(format!("Extension {} not found", id)))
        }
    }

    fn get_db_connection(&self) -> Result<Connection, ExtensionError> {
        let data_dir = self.context.app_handle.path().app_data_dir().map_err(|e| ExtensionError::Io(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        let db_path = data_dir.join("app.db");
        Connection::open(db_path).map_err(ExtensionError::Database)
    }

    fn parse_manifest(&self, manifest_path: &Path) -> Result<ExtensionManifest, ExtensionError> {
        let content = std::fs::read_to_string(manifest_path)?;
        let manifest: ExtensionManifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    fn validate_manifest(&self, manifest: &ExtensionManifest) -> Result<(), ExtensionError> {
        if manifest.name.is_empty() {
            return Err(ExtensionError::Validation("Name is required".to_string()));
        }
        if manifest.version.is_empty() {
            return Err(ExtensionError::Validation("Version is required".to_string()));
        }
        if manifest.entry_point.is_empty() {
            return Err(ExtensionError::Validation("Entry point is required".to_string()));
        }

        // Validate permissions
        let valid_permissions = ["filesystem", "network", "database", "ui", "native"];
        for perm in &manifest.permissions {
            if !valid_permissions.contains(&perm.as_str()) {
                return Err(ExtensionError::Validation(format!("Invalid permission: {}", perm)));
            }
        }

        Ok(())
    }

    fn create_extension(&self, id: &str, manifest: ExtensionManifest, path: PathBuf) -> Result<Box<dyn ExtensionImpl>, ExtensionError> {
        // For now, create a stub extension. In real implementation, this would load
        // the actual extension code based on the entry_point
        let extension = StubExtension {
            id: id.to_string(),
            manifest,
            path,
        };
        Ok(Box::new(extension))
    }

    async fn save_extension_to_db(&self, id: &str, manifest: &ExtensionManifest, manifest_path: &Path) -> Result<(), ExtensionError> {
        let conn = self.get_db_connection()?;
        conn.execute(
            "INSERT INTO extensions (id, name, version, author, description, type, entry_point, manifest_path, enabled)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1)",
            [
                id,
                &manifest.name,
                &manifest.version,
                &manifest.author.as_deref().unwrap_or(""),
                &manifest.description.as_deref().unwrap_or(""),
                &manifest.extension_type.to_string(),
                &manifest.entry_point,
                &manifest_path.to_string_lossy(),
            ],
        )?;
        Ok(())
    }

    async fn save_permissions_to_db(&self, extension_id: &str, permissions: &[String]) -> Result<(), ExtensionError> {
        let conn = self.get_db_connection()?;
        for perm in permissions {
            conn.execute(
                "INSERT INTO extension_permissions (extension_id, permission, granted) VALUES (?, ?, 0)",
                [extension_id, perm],
            )?;
        }
        Ok(())
    }

    async fn remove_extension_from_db(&self, id: &str) -> Result<(), ExtensionError> {
        let conn = self.get_db_connection()?;
        conn.execute("DELETE FROM extension_permissions WHERE extension_id = ?", [id])?;
        conn.execute("DELETE FROM extension_settings WHERE extension_id = ?", [id])?;
        conn.execute("DELETE FROM extensions WHERE id = ?", [id])?;
        Ok(())
    }

    async fn update_extension_enabled_in_db(&self, id: &str, enabled: bool) -> Result<(), ExtensionError> {
        let conn = self.get_db_connection()?;
        conn.execute("UPDATE extensions SET enabled = ? WHERE id = ?", rusqlite::params![enabled, id])?;
        Ok(())
    }
}

pub struct StoreManager {
    sources: HashMap<String, StoreSource>,
}

impl StoreManager {
    pub fn new() -> Self {
        let mut sources = HashMap::new();

        // Add default official source
        let official_source = StoreSource {
            id: "official".to_string(),
            name: "Official Arcadia Store".to_string(),
            source_type: StoreSourceType::Official,
            base_url: "https://api.arcadia-app.com/extensions".to_string(),
            enabled: true,
            priority: 0,
        };
        sources.insert(official_source.id.clone(), official_source);

        Self { sources }
    }

    pub fn add_source(&mut self, source: StoreSource) -> Result<(), StoreError> {
        if self.sources.contains_key(&source.id) {
            return Err(StoreError::Validation("Source with this ID already exists".to_string()));
        }
        self.validate_source(&source)?;
        self.sources.insert(source.id.clone(), source);
        Ok(())
    }

    pub fn remove_source(&mut self, id: &str) -> Result<(), StoreError> {
        if id == "official" {
            return Err(StoreError::Validation("Cannot remove official source".to_string()));
        }
        self.sources.remove(id);
        Ok(())
    }

    pub fn update_source(&mut self, source: StoreSource) -> Result<(), StoreError> {
        if !self.sources.contains_key(&source.id) {
            return Err(StoreError::Validation("Source not found".to_string()));
        }
        if source.id == "official" && source.source_type != StoreSourceType::Official {
            return Err(StoreError::Validation("Cannot change type of official source".to_string()));
        }
        self.validate_source(&source)?;
        self.sources.insert(source.id.clone(), source);
        Ok(())
    }

    pub fn get_source(&self, id: &str) -> Option<&StoreSource> {
        self.sources.get(id)
    }

    pub fn list_sources(&self) -> Vec<StoreSource> {
        let mut sources: Vec<_> = self.sources.values().cloned().collect();
        sources.sort_by_key(|s| s.priority);
        sources
    }

    pub fn get_enabled_sources(&self) -> Vec<StoreSource> {
        self.sources.values().filter(|s| s.enabled).cloned().collect()
    }

    fn validate_source(&self, source: &StoreSource) -> Result<(), StoreError> {
        if source.name.trim().is_empty() {
            return Err(StoreError::Validation("Source name cannot be empty".to_string()));
        }
        if source.base_url.trim().is_empty() {
            return Err(StoreError::Validation("Base URL cannot be empty".to_string()));
        }

        // Validate URL format
        if url::Url::parse(&source.base_url).is_err() {
            return Err(StoreError::Validation("Invalid URL format".to_string()));
        }

        // Security validations for custom sources
        if matches!(source.source_type, StoreSourceType::Custom) {
            self.validate_custom_url(&source.base_url)?;
        }

        Ok(())
    }

    fn validate_custom_url(&self, url: &str) -> Result<(), StoreError> {
        // Only allow HTTPS for custom sources
        if !url.starts_with("https://") {
            return Err(StoreError::Security("Custom sources must use HTTPS".to_string()));
        }

        // Check for potentially dangerous URLs
        let blocked_domains = ["localhost", "127.0.0.1", "0.0.0.0", "10.0.0.0/8", "172.16.0.0/12", "192.168.0.0/16"];
        for domain in &blocked_domains {
            if url.contains(domain) {
                return Err(StoreError::Security(format!("Blocked domain: {}", domain)));
            }
        }

        Ok(())
    }
}

#[tauri::command]
pub async fn fetch_store_extensions(
    source_id: String,
    filters: StoreFilters,
    sort: SortOption,
    page: u32,
    limit: u32,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<Vec<StoreExtension>, String> {
    let manager = store_manager.inner().read().await;
    let source = manager.get_source(&source_id).ok_or_else(|| format!("Source {} not found", source_id))?;
    if !source.enabled {
        return Err(format!("Source {} is disabled", source_id));
    }
    let client = ExtensionStoreClient::new();
    client.fetch_extensions(&source.base_url, &filters, &sort, page, limit).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn fetch_extension_details(
    source_id: String,
    extension_id: String,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<StoreExtensionDetails, String> {
    let manager = store_manager.inner().read().await;
    let source = manager.get_source(&source_id).ok_or_else(|| format!("Source {} not found", source_id))?;
    if !source.enabled {
        return Err(format!("Source {} is disabled", source_id));
    }
    let client = ExtensionStoreClient::new();
    client.fetch_extension_details(&source.base_url, &extension_id).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn install_from_store(
    source_id: String,
    extension_id: String,
    extension_manager: tauri::State<'_, Arc<RwLock<ExtensionManager>>>,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<String, String> {
    let store_mgr = store_manager.inner().read().await;
    let source = store_mgr.get_source(&source_id).ok_or_else(|| format!("Source {} not found", source_id))?;
    if !source.enabled {
        return Err(format!("Source {} is disabled", source_id));
    }
    let client = ExtensionStoreClient::new();

    // Fetch details
    let details = client.fetch_extension_details(&source.base_url, &extension_id).await.map_err(|e| e.to_string())?;

    // Download manifest
    let manifest = client.download_manifest(&details.manifest_url).await.map_err(|e| e.to_string())?;

    // Check if extension is already installed
    let manager = extension_manager.inner().read().await;
    let installed_extensions = manager.list_extensions();
    let is_installed = installed_extensions.iter().any(|ext| ext.id == extension_id);

    // If installed, uninstall the old version first
    let mut manager = extension_manager.inner().write().await;
    if is_installed {
        manager.unload_extension(&extension_id).await.map_err(|e| format!("Failed to uninstall old version: {}", e))?;
    }

    // Download package
    let package_data = client.download_extension(&details.package_url, &details.checksum).await.map_err(|e| e.to_string())?;

    // Save package to temp file
    let temp_dir = std::env::temp_dir();
    let package_path = temp_dir.join(format!("{}.zip", extension_id));
    std::fs::write(&package_path, package_data).map_err(|e| e.to_string())?;

    // Extract package (assuming it's a zip with manifest.json at root)
    // For simplicity, assume the package contains the extension files directly
    // In real implementation, extract to a temp dir and find manifest
    let extract_dir = temp_dir.join(format!("extracted_{}", extension_id));
    std::fs::create_dir_all(&extract_dir).map_err(|e| e.to_string())?;
    // TODO: Implement zip extraction
    // For now, assume manifest is downloaded separately

    // Save manifest to extracted dir
    let manifest_path = extract_dir.join("manifest.json");
    let manifest_json = serde_json::to_string(&manifest).map_err(|e| e.to_string())?;
    std::fs::write(&manifest_path, manifest_json).map_err(|e| e.to_string())?;

    // Install using ExtensionManager
    manager.load_extension(&manifest_path).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_store_sources(store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>) -> Result<Vec<StoreSource>, String> {
    let manager = store_manager.inner().read().await;
    Ok(manager.list_sources())
}

#[tauri::command]
pub async fn add_store_source(
    source: StoreSource,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<(), String> {
    let mut manager = store_manager.inner().write().await;
    manager.add_source(source).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn remove_store_source(
    source_id: String,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<(), String> {
    let mut manager = store_manager.inner().write().await;
    manager.remove_source(&source_id).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_store_source(
    source: StoreSource,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<(), String> {
    let mut manager = store_manager.inner().write().await;
    manager.update_source(source).map_err(|e| e.to_string())
}

pub struct ExtensionRegistry {
    extensions: HashMap<String, ExtensionInfo>,
}

impl ExtensionRegistry {
    pub fn new() -> Self {
        Self {
            extensions: HashMap::new(),
        }
    }

    pub fn register(&mut self, extension: ExtensionInfo) {
        self.extensions.insert(extension.id.clone(), extension);
    }

    pub fn unregister(&mut self, id: &str) {
        self.extensions.remove(id);
    }

    pub fn get(&self, id: &str) -> Option<&ExtensionInfo> {
        self.extensions.get(id)
    }

    pub fn get_all(&self) -> Vec<ExtensionInfo> {
        self.extensions.values().cloned().collect()
    }

    pub fn get_enabled(&self) -> Vec<ExtensionInfo> {
        self.extensions.values().filter(|e| e.enabled).cloned().collect()
    }
}

// Stub extension implementation for demonstration
pub struct StubExtension {
    pub id: String,
    pub manifest: ExtensionManifest,
    pub path: PathBuf,
}

#[async_trait]
impl ExtensionImpl for StubExtension {
    async fn initialize(&mut self, _context: &ExtensionContext) -> Result<(), ExtensionError> {
        println!("Initializing extension: {}", self.manifest.name);
        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), ExtensionError> {
        println!("Shutting down extension: {}", self.manifest.name);
        Ok(())
    }

    async fn handle_hook(&self, hook: &str, params: Value) -> Result<Value, ExtensionError> {
        println!("Extension {} handling hook: {}", self.manifest.name, hook);
        // Stub implementation - return the params as-is
        Ok(params)
    }

    fn get_manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }

    fn get_type(&self) -> ExtensionType {
        self.manifest.extension_type.clone()
    }

    fn get_id(&self) -> &str {
        &self.id
    }
}

pub struct ExtensionStoreClient {
    client: Client,
}

impl ExtensionStoreClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn fetch_extensions(&self, base_url: &str, filters: &StoreFilters, sort: &SortOption, page: u32, limit: u32) -> Result<Vec<StoreExtension>, StoreError> {
        let mut url = format!("{}/extensions?page={}&limit={}", base_url, page, limit);

        if let Some(ext_type) = &filters.extension_type {
            url.push_str(&format!("&type={}", ext_type.to_string()));
        }
        if let Some(tags) = &filters.tags {
            url.push_str(&format!("&tags={}", tags.join(",")));
        }
        if let Some(search) = &filters.search {
            url.push_str(&format!("&search={}", urlencoding::encode(search)));
        }
        url.push_str(&format!("&sort={}", match sort {
            SortOption::Name => "name",
            SortOption::DownloadCount => "downloads",
            SortOption::Rating => "rating",
            SortOption::Newest => "newest",
        }));

        let response = self.client.get(&url).send().await?;
        let extensions: Vec<StoreExtension> = response.json().await?;
        Ok(extensions)
    }

    pub async fn fetch_extension_details(&self, base_url: &str, id: &str) -> Result<StoreExtensionDetails, StoreError> {
        let url = format!("{}/extensions/{}", base_url, id);
        let response = self.client.get(&url).send().await?;
        let details: StoreExtensionDetails = response.json().await?;
        Ok(details)
    }

    pub async fn download_manifest(&self, manifest_url: &str) -> Result<ExtensionManifest, StoreError> {
        let response = self.client.get(manifest_url).send().await?;
        let manifest: ExtensionManifest = response.json().await?;
        self.validate_manifest_security(&manifest)?;
        Ok(manifest)
    }

    pub async fn download_extension(&self, package_url: &str, checksum: &str) -> Result<Vec<u8>, StoreError> {
        let response = self.client.get(package_url).send().await?;
        let bytes = response.bytes().await?;
        let data = bytes.to_vec();

        // Validate checksum
        let computed_checksum = format!("{:x}", md5::compute(&data));
        if computed_checksum != checksum {
            return Err(StoreError::Security("Checksum mismatch".to_string()));
        }

        Ok(data)
    }

    fn validate_manifest_security(&self, manifest: &ExtensionManifest) -> Result<(), StoreError> {
        // Basic security validations
        if manifest.name.contains("..") || manifest.name.contains("/") {
            return Err(StoreError::Security("Invalid extension name".to_string()));
        }
        if manifest.entry_point.contains("..") {
            return Err(StoreError::Security("Invalid entry point".to_string()));
        }
        // Check for dangerous permissions
        let dangerous_perms = ["filesystem", "native"];
        for perm in &manifest.permissions {
            if dangerous_perms.contains(&perm.as_str()) {
                return Err(StoreError::Security(format!("Dangerous permission requested: {}", perm)));
            }
        }
        Ok(())
    }
}