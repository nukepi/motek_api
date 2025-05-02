use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SharedNote {
    pub user_id: String,
    pub note_id: String,
    pub role: String,
    pub granted_at: i64,
}

/// Fetches all shared notes for the given user.
pub async fn list_shared_notes(user_id: String) -> Vec<SharedNote> {
    let endpoint = format!("/api/shared_notes?user_id={}", user_id);
    authorized_get(&endpoint).await.unwrap_or_default()
}

/// Fetches a shared note by note_id and user_id.
pub async fn get_shared_note(note_id: String, user_id: String) -> Option<SharedNote> {
    let endpoint = format!("/api/shared_notes/{}/{}", note_id, user_id);
    authorized_get(&endpoint).await
}

/// Creates a new shared note.
pub async fn create_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    let payload = serde_json::json!({
        "note_id": note_id,
        "user_id": user_id,
        "role": role,
    });
    authorized_post("/api/shared_notes", payload).await
}

/// Updates the role for a shared note.
pub async fn update_shared_note(note_id: String, user_id: String, role: String) -> Option<SharedNote> {
    let payload = serde_json::json!({ "role": role });
    let endpoint = format!("/api/shared_notes/{}/{}", note_id, user_id);
    authorized_put(&endpoint, payload).await
}

/// Deletes a shared note by note_id and user_id.
pub async fn delete_shared_note(note_id: String, user_id: String) -> bool {
    let endpoint = format!("/api/shared_notes/{}/{}", note_id, user_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
