use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attachment {
    pub id: String,
    pub note_id: String,
    pub filename: String,
    pub url: String,
    pub created_at: i64,
}

/// Fetches the list of all attachments for the current user.
pub async fn list_attachments() -> Vec<Attachment> {
    authorized_get("/api/attachments").await.unwrap_or_default()
}

/// Fetches a single attachment by its ID.
pub async fn get_attachment(attachment_id: String) -> Option<Attachment> {
    let endpoint = format!("/api/attachments/{}", attachment_id);
    authorized_get(&endpoint).await
}

/// Creates a new attachment for a note.
pub async fn create_attachment(note_id: String, filename: String, url: String) -> Option<Attachment> {
    let payload = serde_json::json!({
        "note_id": note_id,
        "filename": filename,
        "url": url,
    });
    authorized_post("/api/attachments", payload).await
}

/// Updates an existing attachment by its ID.
pub async fn update_attachment(attachment_id: String, filename: Option<String>, url: Option<String>) -> Option<Attachment> {
    let mut payload = serde_json::Map::new();
    if let Some(filename) = filename {
        payload.insert("filename".to_string(), serde_json::json!(filename));
    }
    if let Some(url) = url {
        payload.insert("url".to_string(), serde_json::json!(url));
    }
    let endpoint = format!("/api/attachments/{}", attachment_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes an attachment by its ID.
pub async fn delete_attachment(attachment_id: String) -> bool {
    let endpoint = format!("/api/attachments/{}", attachment_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
