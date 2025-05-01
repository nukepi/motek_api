use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Attachment – an attachment to a note (e.g., image, PDF).
/// Relations:
///   • note_id → notes.id (the note to which the attachment belongs)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Attachment {
    /// UUID of the attachment
    pub id: Uuid,
    /// UUID of the associated note
    pub note_id: Uuid,
    /// Original file name
    pub filename: String,
    /// URL (or path) to the file
    pub url: String,
    /// Timestamp when the attachment was added
    pub created_at: DateTime<Utc>,
}
