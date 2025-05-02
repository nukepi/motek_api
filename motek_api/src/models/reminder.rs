//! Reminder model – for note reminders.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Reminder – a reminder associated with a note.
/// Relations:
///   • note_id → notes.id (the note for which the reminder is set)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Reminder {
    /// UUID of the reminder
    pub id: Uuid,
    /// UUID of the note
    pub note_id: Uuid,
    /// Date/time for the reminder
    pub remind_at: DateTime<Utc>,
    /// Completion status
    pub is_done: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp (triggered)
    pub updated_at: DateTime<Utc>,
}
