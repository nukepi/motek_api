//! Notebook model – user folder for notes (supports hierarchy).

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Notebook – folder for user notes.
/// Relations:
///   • user_id → users.id (folder owner)
///   • parent_id → notebooks.id (optional parent folder, hierarchy)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Notebook {
    /// UUID of the folder
    pub id: Uuid,
    /// UUID of the owner (users.id)
    pub user_id: Uuid,
    /// Folder name
    pub name: String,
    /// Optional UUID of the parent folder
    pub parent_id: Option<Uuid>,
    /// Creation date
    pub created_at: DateTime<Utc>,
    /// Last modification date (triggered)
    pub updated_at: DateTime<Utc>,
}
