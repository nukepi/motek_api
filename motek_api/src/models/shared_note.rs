use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
/*
/// SharedNote – information about sharing a note with other users.
/// Relations:
///   • user_id → users.id (external user)
///   • note_id → notes.id (shared note)
/// */
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct SharedNote {
    /// UUID of the user with whom the note is shared
    pub user_id:    Uuid,
    /// UUID of the shared note
    pub note_id:    Uuid,
    /// Access role: "viewer", "editor", or "owner"
    pub role:       String,
    /// Timestamp when access was granted
    pub granted_at: DateTime<Utc>,
}