use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Notebook – folder na notatki użytkownika.
/// Relacje:
///   • user_id → users.id (właściciel folderu)
///   • parent_id → notebooks.id (opcjonalny folder nadrzędny, hierarchia)
#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Notebook {
    /// UUID folderu
    pub id: Uuid,
    /// UUID właściciela (users.id)
    pub user_id: Uuid,
    /// Nazwa folderu
    pub name: String,
    /// Opcjonalny UUID folderu nadrzędnego
    pub parent_id: Option<Uuid>,
    /// Data utworzenia
    pub created_at: DateTime<Utc>,
    /// Data ostatniej modyfikacji (trigger)
    pub updated_at: DateTime<Utc>,
}
