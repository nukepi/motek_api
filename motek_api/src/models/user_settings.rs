use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// UserSettings – user-specific settings.
/// Relations:
///   • user_id → users.id (settings owner)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserSettings {
    /// UUID of the settings record
    pub id: Uuid,
    /// UUID of the user (users.id)
    pub user_id: Uuid,
    /// Preferred language (e.g., "en", "pl")
    pub lang: String,
    /// UI theme (e.g., "light", "dark")
    pub theme: String,
    /// User's timezone (e.g., "UTC", "Europe/Warsaw")
    pub timezone: String,
    /// Flag indicating if notifications are enabled
    pub notifications_enabled: bool,
    /// Default sort order for notes (e.g., "created_at", "title")
    pub default_sort: String,
    /// Default editor mode (e.g., "plain", "rich", "markdown")
    pub editor_mode: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp (triggered)
    pub updated_at: DateTime<Utc>,
}
