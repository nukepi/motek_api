use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserSettings {
    pub id: String,
    pub user_id: String,
    pub lang: String,
    pub theme: String,
    pub timezone: String,
    pub notifications_enabled: bool,
    pub default_sort: String,
    pub editor_mode: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Fetches user settings for the given user.
pub async fn get_user_settings(user_id: String) -> Option<UserSettings> {
    let endpoint = format!("/api/user_settings?user_id={}", user_id);
    authorized_get(&endpoint).await
}

/// Creates new user settings.
pub async fn create_user_settings(
    user_id: String,
    lang: String,
    theme: String,
    timezone: String,
    notifications_enabled: bool,
    default_sort: String,
    editor_mode: String,
) -> Option<UserSettings> {
    let payload = serde_json::json!({
        "user_id": user_id,
        "lang": lang,
        "theme": theme,
        "timezone": timezone,
        "notifications_enabled": notifications_enabled,
        "default_sort": default_sort,
        "editor_mode": editor_mode,
    });
    authorized_post("/api/user_settings", payload).await
}

/// Updates user settings by their ID.
pub async fn update_user_settings(
    settings_id: String,
    lang: Option<String>,
    theme: Option<String>,
    timezone: Option<String>,
    notifications_enabled: Option<bool>,
    default_sort: Option<String>,
    editor_mode: Option<String>,
) -> Option<UserSettings> {
    let mut payload = serde_json::Map::new();
    if let Some(lang) = lang {
        payload.insert("lang".to_string(), serde_json::json!(lang));
    }
    if let Some(theme) = theme {
        payload.insert("theme".to_string(), serde_json::json!(theme));
    }
    if let Some(timezone) = timezone {
        payload.insert("timezone".to_string(), serde_json::json!(timezone));
    }
    if let Some(notifications_enabled) = notifications_enabled {
        payload.insert("notifications_enabled".to_string(), serde_json::json!(notifications_enabled));
    }
    if let Some(default_sort) = default_sort {
        payload.insert("default_sort".to_string(), serde_json::json!(default_sort));
    }
    if let Some(editor_mode) = editor_mode {
        payload.insert("editor_mode".to_string(), serde_json::json!(editor_mode));
    }
    let endpoint = format!("/api/user_settings/{}", settings_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes user settings by their ID.
pub async fn delete_user_settings(settings_id: String) -> bool {
    let endpoint = format!("/api/user_settings/{}", settings_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
