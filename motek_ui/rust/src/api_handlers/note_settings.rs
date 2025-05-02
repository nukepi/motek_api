use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct NoteSettings {
    pub id: String,
    pub note_id: String,
    pub color: String,
    pub font: String,
    pub view_mode: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Fetches note settings for the given user.
pub async fn list_note_settings(user_id: String) -> Vec<NoteSettings> {
    let endpoint = format!("/api/note_settings?user_id={}", user_id);
    authorized_get(&endpoint).await.unwrap_or_default()
}

/// Fetches note settings by their ID.
pub async fn get_note_settings(settings_id: String) -> Option<NoteSettings> {
    let endpoint = format!("/api/note_settings/{}", settings_id);
    authorized_get(&endpoint).await
}

/// Creates new note settings.
pub async fn create_note_settings(note_id: String, color: String, font: String, view_mode: String) -> Option<NoteSettings> {
    let payload = serde_json::json!({
        "note_id": note_id,
        "color": color,
        "font": font,
        "view_mode": view_mode,
    });
    authorized_post("/api/note_settings", payload).await
}

/// Updates existing note settings.
pub async fn update_note_settings(settings_id: String, color: Option<String>, font: Option<String>, view_mode: Option<String>) -> Option<NoteSettings> {
    let mut payload = serde_json::Map::new();
    if let Some(color) = color {
        payload.insert("color".to_string(), serde_json::json!(color));
    }
    if let Some(font) = font {
        payload.insert("font".to_string(), serde_json::json!(font));
    }
    if let Some(view_mode) = view_mode {
        payload.insert("view_mode".to_string(), serde_json::json!(view_mode));
    }
    let endpoint = format!("/api/note_settings/{}", settings_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes note settings by their ID.
pub async fn delete_note_settings(settings_id: String) -> bool {
    let endpoint = format!("/api/note_settings/{}", settings_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
