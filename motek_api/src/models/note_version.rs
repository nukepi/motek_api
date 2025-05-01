use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

/// NoteVersion – history of note changes.
/// Relations:
///   • note_id → notes.id (original note)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NoteVersion {
    /// UUID of the version
    pub id: Uuid,
    /// UUID of the note this version belongs to
    pub note_id: Uuid,
    /// Sequential version number (1, 2, 3…)
    pub version_no: i32,
    /// Snapshot of note content in JSON format
    pub content: Value,
    /// Version creation timestamp
    pub created_at: DateTime<Utc>,
}
