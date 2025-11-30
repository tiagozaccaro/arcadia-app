use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::path::PathBuf;

// Re-export the extension types from the main app
// In a real extension, these would be imported from the extension SDK
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
            _ => ExtensionType::Theme,
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
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtensionApis {
    pub provided: Option<Vec<String>>,
    pub required: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Game {
    pub id: String,
    pub name: String,
    pub platform: String,
    pub install_path: Option<String>,
    pub executable_path: Option<String>,
    pub cover_image: Option<String>,
    pub description: Option<String>,
    pub last_played: Option<String>,
    pub playtime_minutes: u32,
}

#[async_trait]
pub trait ExtensionImpl: Send + Sync {
    async fn initialize(&mut self, context: &ExtensionContext) -> Result<(), ExtensionError>;
    async fn shutdown(&mut self) -> Result<(), ExtensionError>;
    async fn handle_hook(&self, hook: &str, params: Value) -> Result<Value, ExtensionError>;
    fn get_manifest(&self) -> &ExtensionManifest;
    fn get_type(&self) -> ExtensionType;
}

pub struct ExtensionContext {
    pub app_handle: tauri::AppHandle,
    pub extension_dir: PathBuf,
}

#[derive(Debug)]
pub enum ExtensionError {
    Io(std::io::Error),
    Json(serde_json::Error),
    Database(String),
    Validation(String),
    NotFound(String),
    PermissionDenied(String),
}

impl std::fmt::Display for ExtensionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ExtensionError::Io(e) => write!(f, "IO error: {}", e),
            ExtensionError::Json(e) => write!(f, "JSON error: {}", e),
            ExtensionError::Database(msg) => write!(f, "Database error: {}", msg),
            ExtensionError::Validation(msg) => write!(f, "Validation error: {}", msg),
            ExtensionError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ExtensionError::PermissionDenied(msg) => write!(f, "Permission denied: {}", msg),
        }
    }
}

impl std::error::Error for ExtensionError {}

/// Sample Game Library Extension Implementation
pub struct SampleGameLibraryExtension {
    manifest: ExtensionManifest,
    games: Vec<Game>,
    is_initialized: bool,
}

impl SampleGameLibraryExtension {
    pub fn new(manifest: ExtensionManifest) -> Self {
        Self {
            manifest,
            games: Vec::new(),
            is_initialized: false,
        }
    }

    /// Stub implementation for scanning games
    /// In a real extension, this would scan filesystem for game installations
    async fn scan_games(&mut self, _params: Value) -> Result<Value, ExtensionError> {
        println!("Sample Game Library: Scanning for games...");

        // Create some sample games for demonstration
        let sample_games = vec![
            Game {
                id: "game1".to_string(),
                name: "Sample Game 1".to_string(),
                platform: "PC".to_string(),
                install_path: Some("/games/sample_game_1".to_string()),
                executable_path: Some("/games/sample_game_1/game.exe".to_string()),
                cover_image: Some("/games/sample_game_1/cover.jpg".to_string()),
                description: Some("A sample game for demonstration purposes".to_string()),
                last_played: Some("2024-01-15T10:30:00Z".to_string()),
                playtime_minutes: 120,
            },
            Game {
                id: "game2".to_string(),
                name: "Sample Game 2".to_string(),
                platform: "PC".to_string(),
                install_path: Some("/games/sample_game_2".to_string()),
                executable_path: Some("/games/sample_game_2/launch.exe".to_string()),
                cover_image: Some("/games/sample_game_2/boxart.png".to_string()),
                description: Some("Another sample game with different metadata".to_string()),
                last_played: None,
                playtime_minutes: 0,
            },
        ];

        self.games = sample_games;

        // Store games in database (stub implementation)
        self.store_games_in_database().await?;

        println!("Sample Game Library: Found {} games", self.games.len());

        Ok(serde_json::json!({
            "scanned": self.games.len(),
            "games": self.games
        }))
    }

    /// Stub implementation for getting games from database
    async fn get_games(&self, params: Value) -> Result<Value, ExtensionError> {
        println!("Sample Game Library: Retrieving games from database...");

        // In a real implementation, this would query the database
        // For now, return the cached games
        let limit = params.get("limit").and_then(|v| v.as_u64()).unwrap_or(50) as usize;
        let offset = params.get("offset").and_then(|v| v.as_u64()).unwrap_or(0) as usize;

        let games: Vec<&Game> = self.games.iter()
            .skip(offset)
            .take(limit)
            .collect();

        Ok(serde_json::json!({
            "total": self.games.len(),
            "games": games,
            "limit": limit,
            "offset": offset
        }))
    }

    /// Stub implementation for getting detailed game information
    async fn get_game_details(&self, params: Value) -> Result<Value, ExtensionError> {
        let game_id = params.get("game_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::Validation("game_id parameter required".to_string()))?;

        println!("Sample Game Library: Getting details for game: {}", game_id);

        let game = self.games.iter()
            .find(|g| g.id == game_id)
            .ok_or_else(|| ExtensionError::NotFound(format!("Game with id {} not found", game_id)))?;

        Ok(serde_json::json!(game))
    }

    /// Stub implementation for launching a game
    async fn launch_game(&self, params: Value) -> Result<Value, ExtensionError> {
        let game_id = params.get("game_id")
            .and_then(|v| v.as_str())
            .ok_or_else(|| ExtensionError::Validation("game_id parameter required".to_string()))?;

        println!("Sample Game Library: Launching game: {}", game_id);

        let game = self.games.iter()
            .find(|g| g.id == game_id)
            .ok_or_else(|| ExtensionError::NotFound(format!("Game with id {} not found", game_id)))?;

        // In a real implementation, this would actually launch the game
        // For now, just return success
        println!("Sample Game Library: Would launch game '{}' at path: {:?}",
                 game.name, game.executable_path);

        Ok(serde_json::json!({
            "success": true,
            "game_id": game_id,
            "message": format!("Game '{}' launched successfully", game.name)
        }))
    }

    /// Stub implementation for storing games in database
    async fn store_games_in_database(&self) -> Result<(), ExtensionError> {
        println!("Sample Game Library: Storing {} games in database...", self.games.len());

        // In a real implementation, this would use the database API
        // For demonstration, we'll just log what would be stored
        for game in &self.games {
            println!("  Storing game: {} (ID: {})", game.name, game.id);
        }

        Ok(())
    }

    /// Handle API calls from the extension system
    async fn handle_api_call(&self, api: &str, params: Value) -> Result<Value, ExtensionError> {
        match api {
            "scan_games" => {
                // Note: scan_games modifies state, but this method takes &self
                // In a real implementation, we'd need interior mutability or different design
                Err(ExtensionError::Validation("scan_games requires mutable access".to_string()))
            },
            "get_games" => self.get_games(params).await,
            "get_game_details" => self.get_game_details(params).await,
            "launch_game" => self.launch_game(params).await,
            _ => Err(ExtensionError::NotFound(format!("Unknown API: {}", api))),
        }
    }
}

#[async_trait]
impl ExtensionImpl for SampleGameLibraryExtension {
    async fn initialize(&mut self, _context: &ExtensionContext) -> Result<(), ExtensionError> {
        println!("Sample Game Library Extension: Initializing...");

        // Perform any setup needed for the extension
        // In a real extension, this might include:
        // - Loading configuration
        // - Setting up database connections
        // - Initializing caches

        self.is_initialized = true;
        println!("Sample Game Library Extension: Initialized successfully");

        Ok(())
    }

    async fn shutdown(&mut self) -> Result<(), ExtensionError> {
        println!("Sample Game Library Extension: Shutting down...");

        // Clean up resources
        self.games.clear();
        self.is_initialized = false;

        println!("Sample Game Library Extension: Shutdown complete");
        Ok(())
    }

    async fn handle_hook(&self, hook: &str, params: Value) -> Result<Value, ExtensionError> {
        println!("Sample Game Library Extension: Handling hook: {}", hook);

        match hook {
            "on_startup" => {
                println!("Sample Game Library Extension: App startup detected");
                // Could perform initial setup or validation here
                Ok(serde_json::json!({
                    "status": "ready",
                    "games_count": self.games.len()
                }))
            },
            "on_game_scan" => {
                println!("Sample Game Library Extension: Game scan requested");
                // In a real implementation, this would trigger a scan
                // Since handle_hook takes &self, we can't modify state here
                // We'd need to use a different mechanism (like channels or interior mutability)
                Ok(serde_json::json!({
                    "scan_triggered": true,
                    "message": "Game scan initiated"
                }))
            },
            _ => {
                println!("Sample Game Library Extension: Unknown hook: {}", hook);
                Ok(serde_json::json!({
                    "handled": false,
                    "hook": hook
                }))
            }
        }
    }

    fn get_manifest(&self) -> &ExtensionManifest {
        &self.manifest
    }

    fn get_type(&self) -> ExtensionType {
        ExtensionType::GameLibrary
    }
}

// Factory function to create the extension instance
// This would be called by the extension loading system
pub fn create_extension(manifest: ExtensionManifest) -> Box<dyn ExtensionImpl> {
    Box::new(SampleGameLibraryExtension::new(manifest))
}