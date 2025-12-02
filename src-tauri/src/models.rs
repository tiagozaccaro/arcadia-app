use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Platform {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub icon_path: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub platform_id: i64,
    pub description: Option<String>,
    pub developer: Option<String>,
    pub publisher: Option<String>,
    pub release_date: Option<String>,
    pub cover_image_path: Option<String>,
    pub executable_path: Option<String>,
    pub working_directory: Option<String>,
    pub arguments: Option<String>,
    pub is_favorite: bool,
    pub playtime_minutes: i64,
    pub last_played: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: i64,
    pub name: String,
}