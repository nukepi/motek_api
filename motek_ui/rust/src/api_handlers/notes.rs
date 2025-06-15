use serde::{Serialize, Deserialize, Deserializer};
use chrono::DateTime;
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Debug, Clone)]
pub struct Note {
    pub id: String,
    pub user_id: String,
    pub notebook_id: Option<String>,  // Changed from String to Option<String>
    pub title: String,
    pub content: String,
    pub is_archived: bool,
    pub is_pinned: bool,
    pub tags: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl<'de> Deserialize<'de> for Note {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NoteHelper {
            id: String,
            user_id: String,
            notebook_id: Option<String>,  // Handles null
            title: String,
            content: String,
            is_archived: bool,
            is_pinned: bool,
            tags: serde_json::Value,  // Handles different tag formats
            created_at: String,       // Dates as String
            updated_at: String,       // Dates as String
        }

        let helper = NoteHelper::deserialize(deserializer)?;
        
        // Tags conversion
        let tags = match helper.tags {
            serde_json::Value::Array(arr) => {
                // If it's an array, join elements with commas
                arr.iter()
                   .map(|v| v.as_str().unwrap_or("").to_string())
                   .collect::<Vec<_>>()
                   .join(",")
            },
            serde_json::Value::String(s) => s,
            _ => String::new(),
        };
        
        // Date conversion
        let created_at = DateTime::parse_from_rfc3339(&helper.created_at)
            .map_err(serde::de::Error::custom)?
            .timestamp_millis();
            
        let updated_at = DateTime::parse_from_rfc3339(&helper.updated_at)
            .map_err(serde::de::Error::custom)?
            .timestamp_millis();

        Ok(Note {
            id: helper.id,
            user_id: helper.user_id,
            notebook_id: helper.notebook_id,
            title: helper.title,
            content: helper.content,
            is_archived: helper.is_archived,
            is_pinned: helper.is_pinned,
            tags,
            created_at,
            updated_at,
        })
    }
}

/// Fetches the list of all notes for the current user.
pub async fn list_notes() -> Vec<Note> {
    authorized_get("/api/notes").await.unwrap_or_default()
}

/// Fetches a single note by its ID.
pub async fn get_note(note_id: String) -> Option<Note> {
    let endpoint = format!("/api/notes/{}", note_id);
    authorized_get(&endpoint).await
}

/// Creates a new note with the given title and content.
pub async fn create_note(title: String, content: String) -> Option<Note> {
    let payload = serde_json::json!({
        "title": title,
        "content": content,
        "tags": [] ,
    });
    authorized_post("/api/notes", payload).await
}

/// Updates an existing note by its ID. Only provided fields will be updated.
pub async fn update_note(note_id: String, title: Option<String>, content: Option<String>) -> Option<Note> {
    let mut payload = serde_json::Map::new();
    if let Some(title) = title { payload.insert("title".to_string(), serde_json::json!(title)); }
    if let Some(content) = content { payload.insert("content".to_string(), serde_json::json!(content)); }
    let endpoint = format!("/api/notes/{}", note_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes a note by its ID.
pub async fn delete_note(note_id: String) -> bool {
    let endpoint = format!("/api/notes/{}", note_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
