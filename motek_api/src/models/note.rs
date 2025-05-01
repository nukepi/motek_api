use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

/// Note – the main note entity.
/// Relations:
///   • user_id → users.id (note owner)
///   • notebook_id → notebooks.id (optional containing folder)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Note {
    /// UUID of the note
    pub id: Uuid,
    /// UUID of the owner (users.id)
    pub user_id: Uuid,
    /// UUID of the folder (notebooks.id), if assigned
    pub notebook_id: Option<Uuid>,
    /// Title of the note
    pub title: String,
    /// Content in JSON (block editor / markdown, etc.)
    pub content: Value,
    /// Archive flag
    pub is_archived: bool,
    /// Pin flag
    pub is_pinned: bool,
    /// Array of tags in JSONB
    pub tags: Value,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp (triggered)
    pub updated_at: DateTime<Utc>,
}
