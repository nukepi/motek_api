use axum::{
    Router,
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::reminder::Reminder;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",      get(list_reminders).post(create_reminder))
        .route("/{id}",   get(get_one).put(update).delete(delete_one))
}

pub async fn list_reminders(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Reminder>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, Reminder>("SELECT * FROM reminders")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateReminder {
    pub note_id:   Uuid,
    pub remind_at: DateTime<Utc>,
}

pub async fn create_reminder(
    State(state): State<AppState>,
    Json(p): Json<CreateReminder>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    let r = sqlx::query_as::<_, Reminder>(
            "INSERT INTO reminders (note_id,remind_at) \
             VALUES ($1,$2) RETURNING *"
        )
        .bind(p.note_id)
        .bind(p.remind_at)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(r)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, Reminder>(
            "SELECT * FROM reminders WHERE id=$1"
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Some(r) = opt {
        Ok((StatusCode::OK, Json(r)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateReminder {
    pub remind_at: Option<DateTime<Utc>>,
    pub is_done:   Option<bool>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p):  Json<UpdateReminder>,
) -> Result<(StatusCode, Json<Reminder>), (StatusCode, String)> {
    sqlx::query(
        r#"UPDATE reminders SET
            remind_at = COALESCE($2, remind_at),
            is_done   = COALESCE($3, is_done)
          WHERE id = $1"#
    )
    .bind(id)
    .bind(p.remind_at)
    .bind(p.is_done)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    get_one(State(state), Path(id)).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM reminders WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
