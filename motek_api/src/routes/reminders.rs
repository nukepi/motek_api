use crate::models::reminder::Reminder;
use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

/// Returns a router for reminders endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_reminders).post(create_reminder))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

/// List all reminders for notes owned by the user.
pub async fn list_reminders(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<Reminder>>), (StatusCode, String)> {
    info!("User {} requested reminders list", user_id);
    let rows = sqlx::query_as::<_, Reminder>(
        "SELECT r.* FROM reminders r
         JOIN notes n ON r.note_id = n.id
         WHERE n.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error fetching reminders for user {}: {}", user_id, e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateReminder {
    pub note_id: Uuid,
    pub remind_at: DateTime<Utc>,
}

/// Create a reminder for a note.
/// Should check if the user owns the note.
pub async fn create_reminder(
    State(state): State<AppState>,
    Json(p): Json<CreateReminder>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    info!("Creating reminder for note_id {}", p.note_id);
    let r = sqlx::query_as::<_, Reminder>(
        "INSERT INTO reminders (note_id,remind_at) \
             VALUES ($1,$2) RETURNING *",
    )
    .bind(p.note_id)
    .bind(p.remind_at)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!(
            "DB error creating reminder for note_id {}: {}",
            p.note_id, e
        );
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;
    Ok((StatusCode::CREATED, Json(r)))
}

/// Get a single reminder by id.
/// Should check if the user owns the note.
pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    info!("Fetching reminder with id {}", id);
    let opt = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders WHERE id=$1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error fetching reminder {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;
    if let Some(r) = opt {
        Ok((StatusCode::OK, Json(r)))
    } else {
        info!("Reminder with id {} not found", id);
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateReminder {
    pub remind_at: Option<DateTime<Utc>>,
    pub is_done: Option<bool>,
}

/// Update a reminder by id.
/// Should check if the user owns the note.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateReminder>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    info!("Updating reminder id {}", id);
    sqlx::query(
        r#"UPDATE reminders SET
            remind_at = COALESCE($2, remind_at),
            is_done   = COALESCE($3, is_done)
          WHERE id = $1"#,
    )
    .bind(id)
    .bind(p.remind_at)
    .bind(p.is_done)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error updating reminder {}: {}", id, e);
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;
    get_one(State(state), Path(id)).await
}

/// Delete a reminder by id.
/// Should check if the user owns the note.
pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Deleting reminder id {}", id);
    sqlx::query("DELETE FROM reminders WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error deleting reminder {}: {}", id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;
    Ok(StatusCode::NO_CONTENT)
}
