//! User model – represents a registered user.

use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User – a registered user of the application.
/// Table: users
///   • id: primary key, auto-increment
#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    /// Unique identifier for the user
    pub id: Uuid,
    /// User's email address (must be unique)
    pub email: String,
    /// Hashed password for authentication
    pub password: String,
    /// Timestamp when the user account was created
    pub created_at: NaiveDateTime,
}
