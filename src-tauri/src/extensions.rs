use arcadia_extension_framework::models::*;
use arcadia_extension_framework::traits::{ExtensionImpl, ExtensionContext};
use arcadia_extension_framework::error::*;
use arcadia_extension_framework::manifest;
use arcadia_extension_framework::registry::ExtensionRegistry;
use arcadia_extension_framework::store::models::*;
use arcadia_extension_framework::store::manager::StoreManager;
use arcadia_extension_framework::store::client::ExtensionStoreClient;
use serde::Serialize;
use async_trait::async_trait;

#[derive(Serialize)]
pub struct FrontendStoreExtension {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub extension_type: ExtensionType,
    pub source_id: String,
    pub icon: Option<String>,
    pub download_count: u32,
    pub rating: f32,
    pub tags: Vec<String>,
}
use rusqlite::Connection;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use uuid::Uuid;



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
        self.save_extension_to_db(&id, extension.get_manifest(), manifest_path).await?;

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

    #[allow(unused)]
    pub async fn call_hook(&self, hook: &str, params: Value) -> Result<Vec<Value>, ExtensionError> {
        let mut results = Vec::new();
        for extension in self.extensions.values() {
            if let Ok(result) = extension.handle_hook(hook, params.clone()).await {
                results.push(result);
            }
        }
        Ok(results)
    }

    pub fn get_extension(&self, id: &str) -> Option<&dyn ExtensionImpl> {
        self.extensions.get(id).map(|boxed| boxed.as_ref())
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
        if let Some(extension_info) = self.registry.get_mut(id) {
            extension_info.enabled = true;
            self.update_extension_enabled_in_db(id, true).await?;
            Ok(())
        } else {
            Err(ExtensionError::NotFound(format!("Extension {} not found", id)))
        }
    }

    pub async fn disable_extension(&mut self, id: &str) -> Result<(), ExtensionError> {
        if let Some(extension_info) = self.registry.get_mut(id) {
            extension_info.enabled = false;
            self.update_extension_enabled_in_db(id, false).await?;
            Ok(())
        } else {
            Err(ExtensionError::NotFound(format!("Extension {} not found", id)))
        }
    }

    fn get_db_connection(&self) -> Result<Connection, ExtensionError> {
        let data_dir = self.context.app_handle.path().app_data_dir().map_err(|e| ExtensionError::Io(std::io::Error::other(e.to_string())))?;
        let db_path = data_dir.join("app.db");
        Connection::open(db_path).map_err(ExtensionError::Database)
    }

    fn parse_manifest(&self, manifest_path: &Path) -> Result<ExtensionManifest, ExtensionError> {
        manifest::parse_manifest(manifest_path)
    }

    fn validate_manifest(&self, manifest: &ExtensionManifest) -> Result<(), ExtensionError> {
        manifest::validate_manifest(manifest)
    }

    fn create_extension(&self, id: &str, manifest: ExtensionManifest, path: PathBuf) -> Result<Box<dyn ExtensionImpl>, ExtensionError> {
        // For now, create a stub extension. In real implementation, this would load
        // the actual extension code based on the entry_point
        let extension = StubExtension {
            id: id.to_string(),
            manifest,
            _path: path,
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
                manifest.author.as_deref().unwrap_or(""),
                manifest.description.as_deref().unwrap_or(""),
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

#[derive(Deserialize)]
struct DefaultExtension {
    name: String,
    description: String,
    version: String,
    author: String,
    tags: Vec<String>,
    icon: Option<String>,
    manifest_url: String,
}

#[derive(Deserialize, Debug)]
pub struct FrontendStoreFilters {
    extension_type: Option<ExtensionType>,
    tags: Option<Vec<String>>,
    search: Option<String>,
    source_ids: Option<Vec<String>>,
}

async fn load_default_extensions(_app_handle: &tauri::AppHandle) -> Result<Vec<FrontendStoreExtension>, String> {
    println!("Loading default extensions from remote Arcadia Store source");
    // Return empty vec since extensions should be loaded from sources
    Ok(vec![])
}

#[tauri::command]
pub async fn fetch_store_extensions(
    app_handle: tauri::AppHandle,
    filters: FrontendStoreFilters,
    sort: SortOption,
    page: u32,
    limit: u32,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<Vec<FrontendStoreExtension>, String> {
    println!("fetch_store_extensions called with page: {}, limit: {}", page, limit);
    println!("Filters: {:?}", filters);
    println!("Sort: {:?}", sort);
    let default_exts = load_default_extensions(&app_handle).await?;
    println!("Loaded {} default extensions", default_exts.len());
    let mut results = default_exts;

    if let Some(source_ids) = &filters.source_ids {
        println!("Processing {} source IDs", source_ids.len());
        for source_id in source_ids {
            println!("Processing source: {}", source_id);
            let manager = store_manager.inner().read().await;
            let source = manager.get_source(source_id).ok_or_else(|| format!("Source {} not found", source_id))?;
            if !source.enabled {
                println!("Source {} is disabled, skipping", source_id);
                continue;
            }

            if source_id == "default" {
                // For the default source, load extensions directly from the JSON file
                println!("Loading extensions from default source JSON file");
                let response = reqwest::get(&source.base_url).await.map_err(|e| format!("Failed to download manifest: {}", e))?;
                let manifest_content = response.text().await.map_err(|e| format!("Failed to read response: {}", e))?;
                let default_exts: Vec<DefaultExtension> = serde_json::from_str(&manifest_content).map_err(|e| format!("Failed to parse manifest: {}", e))?;
                println!("Parsed {} extensions from default source", default_exts.len());

                let frontend_results: Vec<FrontendStoreExtension> = default_exts.into_iter().map(|ext| FrontendStoreExtension {
                    id: ext.manifest_url.clone(),
                    name: ext.name,
                    description: ext.description,
                    version: ext.version,
                    author: ext.author,
                    extension_type: ExtensionType::GameLibrary,
                    source_id: source_id.clone(),
                    icon: ext.icon,
                    download_count: 0,
                    rating: 0.0,
                    tags: ext.tags,
                }).collect();
                results.extend(frontend_results);
            } else {
                // For other sources, use the API client
                let client = ExtensionStoreClient::new();
                let api_filters = StoreFilters {
                    extension_type: filters.extension_type.clone(),
                    tags: filters.tags.clone(),
                    search: filters.search.clone(),
                };
                let source_results = client.fetch_extensions(&source.base_url, &api_filters, &sort, page, limit).await.map_err(|e| e.to_string())?;
                println!("Fetched {} extensions from source {}", source_results.len(), source_id);
                let frontend_results: Vec<FrontendStoreExtension> = source_results.into_iter().map(|ext| FrontendStoreExtension {
                    id: ext.id,
                    name: ext.name,
                    description: ext.description,
                    version: ext.version,
                    author: ext.author,
                    extension_type: ext.extension_type,
                    source_id: source_id.clone(),
                    icon: None, // External sources don't provide icons
                    download_count: ext.download_count,
                    rating: ext.rating,
                    tags: ext.tags,
                }).collect();
                results.extend(frontend_results);
            }
        }
    } else {
        println!("No source_ids provided in filters");
    }

    println!("Returning {} total extensions", results.len());
    Ok(results)
}

#[tauri::command]
pub async fn fetch_extension_details(
    source_id: String,
    extension_id: String,
    store_manager: tauri::State<'_, Arc<RwLock<StoreManager>>>,
) -> Result<StoreExtensionDetails, String> {
    if source_id == "default" {
        // For default extensions, download the manifest from the extension_id (which is the manifest_url)
        let client = ExtensionStoreClient::new();
        let manifest: ExtensionManifest = client.download_manifest(&extension_id).await.map_err(|e| e.to_string())?;
        let details = StoreExtensionDetails {
            id: extension_id.clone(),
            name: manifest.name,
            version: manifest.version,
            author: manifest.author.unwrap_or_default(),
            description: manifest.description.unwrap_or_default(),
            extension_type: manifest.extension_type,
            download_count: 0,
            rating: 0.0,
            tags: vec![],
            manifest_url: extension_id.clone(),
            package_url: "".to_string(),
            checksum: "".to_string(),
            readme: "".to_string(),
            screenshots: vec![],
            dependencies: manifest.dependencies.unwrap_or_default(),
        };
        Ok(details)
    } else {
        let manager = store_manager.inner().read().await;
        let source = manager.get_source(&source_id).ok_or_else(|| format!("Source {} not found", source_id))?;
        if !source.enabled {
            return Err(format!("Source {} is disabled", source_id));
        }
        let client = ExtensionStoreClient::new();
        client.fetch_extension_details(&source.base_url, &extension_id).await.map_err(|e| e.to_string())
    }
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


// Stub extension implementation for demonstration
pub struct StubExtension {
    pub id: String,
    pub manifest: ExtensionManifest,
    pub _path: PathBuf,
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
