use crate::models::*;
use async_trait::async_trait;
use rusqlite::Connection;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Manager};
use tokio::sync::RwLock;
use uuid::Uuid;

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