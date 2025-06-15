use serde::{Serialize, Deserialize, Deserializer};
use chrono::DateTime;
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Debug, Clone)]
pub struct Notebook {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

impl<'de> Deserialize<'de> for Notebook {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct NotebookHelper {
            id: String,
            user_id: String,
            name: String,
            parent_id: Option<String>,
            created_at: String,
            updated_at: String,
        }

        let helper = NotebookHelper::deserialize(deserializer)?;
        
        // Konwersja ciągów znaków na timestamp
        let created_at = DateTime::parse_from_rfc3339(&helper.created_at)
            .map_err(serde::de::Error::custom)?
            .timestamp_millis();
            
        let updated_at = DateTime::parse_from_rfc3339(&helper.updated_at)
            .map_err(serde::de::Error::custom)?
            .timestamp_millis();

        Ok(Notebook {
            id: helper.id,
            user_id: helper.user_id,
            name: helper.name,
            parent_id: helper.parent_id,
            created_at,
            updated_at,
        })
    }
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
