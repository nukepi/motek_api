use chrono::DateTime;
use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Notebook {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub parent_id: String,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Fetches the list of all notebooks for the current user.
pub async fn list_notebooks() -> Vec<Notebook> {
    authorized_get("/api/notebooks").await.unwrap_or_default()
}

/// Fetches a single notebook by its ID.
pub async fn get_notebook(notebook_id: String) -> Option<Notebook> {
    let endpoint = format!("/api/notebooks/{}", notebook_id);
    authorized_get(&endpoint).await
}

/// Creates a new notebook with the given name and optional parent_id.
pub async fn create_notebook(name: String, parent_id: Option<String>) -> Option<Notebook> {
    let payload = match parent_id {
        Some(pid) => serde_json::json!({ "name": name, "parent_id": pid }),
        None => serde_json::json!({ "name": name }),
    };
    authorized_post("/api/notebooks", payload).await
}

/// Updates an existing notebook by its ID. Only provided fields will be updated.
pub async fn update_notebook(notebook_id: String, name: Option<String>, parent_id: Option<String>) -> Option<Notebook> {
    let mut payload = serde_json::Map::new();
    if let Some(name) = name {
        payload.insert("name".to_string(), serde_json::json!(name));
    }
    if let Some(parent_id) = parent_id {
        payload.insert("parent_id".to_string(), serde_json::json!(parent_id));
    }
    let endpoint = format!("/api/notebooks/{}", notebook_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes a notebook by its ID.
pub async fn delete_notebook(notebook_id: String) -> bool {
    let endpoint = format!("/api/notebooks/{}", notebook_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
