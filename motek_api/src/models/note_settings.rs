use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// NoteSettings – settings specific to a single note.
/// Relations:
///   • note_id → notes.id (one settings record per note)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct NoteSettings {
    /// UUID of the settings record
    pub id:          Uuid,
    /// UUID of the note these settings apply to
    pub note_id:     Uuid,
    /// Display color for the note (e.g., "#ffffff")
    pub color:       String,
    /// Font for rendering (e.g., "sans-serif")
    pub font:        String,
    /// View mode: "plain" / "rich" / "markdown"
    pub view_mode:   String,
    /// Creation timestamp
    pub created_at:  DateTime<Utc>,
    /// Last update timestamp (triggered)
    pub updated_at:  DateTime<Utc>,
}