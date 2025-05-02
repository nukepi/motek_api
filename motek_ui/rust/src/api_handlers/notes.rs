use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Note {
    pub id: String,
    pub user_id: String,
    pub notebook_id: String,
    pub title: String,
    pub content: String,
    pub is_archived: bool,
    pub is_pinned: bool,
    pub tags: String,
    pub created_at: i64,
    pub updated_at: i64,
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
