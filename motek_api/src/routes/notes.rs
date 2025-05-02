use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use uuid::Uuid;

use crate::models::note::Note;

/// Returns a router for note endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_notes).post(create_note))
        .route("/{id}", get(get_note).put(update_note).delete(delete_note))
}

#[derive(Serialize)]
pub struct NotesListResponse(Vec<Note>);

/// List all notes for a user.
pub async fn list_notes(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<NotesListResponse>), (StatusCode, String)> {
    info!("User {} requested notes list", user_id);
    let notes = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes WHERE user_id = $1 ORDER BY updated_at DESC",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error fetching notes for user {}: {}", user_id, e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    Ok((StatusCode::OK, Json(NotesListResponse(notes))))
}

#[derive(Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub content: serde_json::Value,
    pub notebook_id: Option<Uuid>,
    pub tags: serde_json::Value,
}

/// Create a new note for a user.
pub async fn create_note(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<CreateNotePayload>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    info!(
        "User {} is creating a note with title '{}'",
        user_id, payload.title
    );
    let note = sqlx::query_as::<_, Note>(
        "INSERT INTO notes (user_id, notebook_id, title, content, tags)
         VALUES ($1,$2,$3,$4,$5) RETURNING *",
    )
    .bind(user_id)
    .bind(payload.notebook_id)
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(&payload.tags)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error creating note for user {}: {}", user_id, e);
        (StatusCode::BAD_REQUEST, e.to_string())
    })?;

    Ok((StatusCode::CREATED, Json(note)))
}

/// Get one note by id for a user.
pub async fn get_note(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    info!("User {} is fetching note id {}", user_id, id);
    let opt = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error fetching note {} for user {}: {}", id, user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if let Some(n) = opt {
        Ok((StatusCode::OK, Json(n)))
    } else {
        info!("Note {} not found for user {}", id, user_id);
        Err((StatusCode::NOT_FOUND, "Note does not exist".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateNotePayload {
    pub title: Option<String>,
    pub content: Option<serde_json::Value>,
    pub is_archived: Option<bool>,
    pub is_pinned: Option<bool>,
    pub tags: Option<serde_json::Value>,
}

/// Update a note for a user.
pub async fn update_note(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNotePayload>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    info!("User {} is updating note id {}", user_id, id);
    let result = sqlx::query(
        r#"UPDATE notes SET
            title       = COALESCE($2, title),
            content     = COALESCE($3, content),
            is_archived = COALESCE($4, is_archived),
            is_pinned   = COALESCE($5, is_pinned),
            tags        = COALESCE($6, tags)
          WHERE id = $1 AND user_id = $7"#,
    )
    .bind(id)
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.is_archived)
    .bind(payload.is_pinned)
    .bind(payload.tags)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error updating note {} for user {}: {}", id, user_id, e);
        (StatusCode::BAD_REQUEST, e.to_string())
    })?;

    if result.rows_affected() == 0 {
        info!("Note {} not found for user {}", id, user_id);
        return Err((StatusCode::NOT_FOUND, "Note does not exist".to_string()));
    }

    get_note(State(state), AuthUser(user_id), Path(id)).await
}

/// Delete a note for a user.
pub async fn delete_note(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("User {} is deleting note id {}", user_id, id);
    let result = sqlx::query("DELETE FROM notes WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error deleting note {} for user {}: {}", id, user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    if result.rows_affected() == 0 {
        info!("Note {} not found for user {}", id, user_id);
        return Err((StatusCode::NOT_FOUND, "Note does not exist".to_string()));
    }

    Ok(StatusCode::NO_CONTENT)
}
