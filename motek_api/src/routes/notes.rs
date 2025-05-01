use axum::{
    Router,
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;
use crate::models::note::Note;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_notes).post(create_note))
        .route("/{id}", get(get_note).put(update_note).delete(delete_note))
}

#[derive(Serialize)]
pub struct NotesListResponse(Vec<Note>);

pub async fn list_notes(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<NotesListResponse>), (StatusCode, String)> {
    let notes = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(NotesListResponse(notes))))
}

#[derive(Deserialize)]
pub struct CreateNotePayload {
    pub title:       String,
    pub content:     serde_json::Value,
    pub notebook_id: Option<Uuid>,
    pub tags:        serde_json::Value,
}

pub async fn create_note(
    State(state): State<AppState>,
    Json(payload): Json<CreateNotePayload>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    let note = sqlx::query_as::<_, Note>(
            r#"INSERT INTO notes (user_id, notebook_id, title, content, tags)
               VALUES ($1,$2,$3,$4,$5) RETURNING *"#
        )
        // TODO: tu podstaw aktualnego usera
        .bind(Uuid::new_v4())
        .bind(payload.notebook_id)
        .bind(&payload.title)
        .bind(&payload.content)
        .bind(&payload.tags)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, Note>("SELECT * FROM notes WHERE id = $1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Some(n) = opt {
        Ok((StatusCode::OK, Json(n)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateNotePayload {
    pub title:       Option<String>,
    pub content:     Option<serde_json::Value>,
    pub is_archived: Option<bool>,
    pub is_pinned:   Option<bool>,
    pub tags:        Option<serde_json::Value>,
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateNotePayload>,
) -> Result<(StatusCode, Json<Note>), (StatusCode, String)> {
    sqlx::query(
        r#"UPDATE notes SET
            title       = COALESCE($2, title),
            content     = COALESCE($3, content),
            is_archived = COALESCE($4, is_archived),
            is_pinned   = COALESCE($5, is_pinned),
            tags        = COALESCE($6, tags)
          WHERE id = $1"#
    )
    .bind(id)
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.is_archived)
    .bind(payload.is_pinned)
    .bind(payload.tags)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    get_note(State(state), Path(id)).await
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
