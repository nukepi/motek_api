use serde::{Serialize, Deserialize};
use crate::utils::helpers::{authorized_get, authorized_post, authorized_put, authorized_delete};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reminder {
    pub id: String,
    pub note_id: String,
    pub remind_at: i64,
    pub is_done: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

/// Fetches all reminders for the given user.
pub async fn list_reminders(user_id: String) -> Vec<Reminder> {
    let endpoint = format!("/api/reminders?user_id={}", user_id);
    authorized_get(&endpoint).await.unwrap_or_default()
}

/// Fetches a reminder by its ID.
pub async fn get_reminder(reminder_id: String) -> Option<Reminder> {
    let endpoint = format!("/api/reminders/{}", reminder_id);
    authorized_get(&endpoint).await
}

/// Creates a new reminder.
pub async fn create_reminder(note_id: String, remind_at: i64) -> Option<Reminder> {
    let payload = serde_json::json!({
        "note_id": note_id,
        "remind_at": remind_at,
    });
    authorized_post("/api/reminders", payload).await
}

/// Updates an existing reminder by its ID.
pub async fn update_reminder(reminder_id: String, is_done: Option<bool>, remind_at: Option<i64>) -> Option<Reminder> {
    let mut payload = serde_json::Map::new();
    if let Some(is_done) = is_done {
        payload.insert("is_done".to_string(), serde_json::json!(is_done));
    }
    if let Some(remind_at) = remind_at {
        payload.insert("remind_at".to_string(), serde_json::json!(remind_at));
    }
    let endpoint = format!("/api/reminders/{}", reminder_id);
    authorized_put(&endpoint, serde_json::Value::Object(payload)).await
}

/// Deletes a reminder by its ID.
pub async fn delete_reminder(reminder_id: String) -> bool {
    let endpoint = format!("/api/reminders/{}", reminder_id);
    authorized_delete(&endpoint).await.unwrap_or(false)
}
