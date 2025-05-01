use axum::{
    Router,
    extract::{State, Path, Json},
    routing::get,
    http::StatusCode
};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::attachment::Attachment;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",      get(list_attachments).post(create_attachment))
        .route("/{id}",   get(fetch_attachment).delete(remove_attachment))
}

// --- HANDLERY ---

pub async fn list_attachments(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Attachment>>), (StatusCode, String)> {
    let items = sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments"
        )
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(items)))
}

#[derive(Deserialize)]
pub struct CreateAttachmentPayload {
    pub note_id:  Uuid,
    pub filename: String,
    pub url:      String,
}

pub async fn create_attachment(
    State(state): State<AppState>,
    Json(payload): Json<CreateAttachmentPayload>,
) -> Result<(StatusCode, Json<Attachment>), (StatusCode, String)> {
    let a = sqlx::query_as::<_, Attachment>(
            "INSERT INTO attachments (note_id, filename, url) \
             VALUES ($1, $2, $3) RETURNING *"
        )
        .bind(payload.note_id)
        .bind(payload.filename)
        .bind(payload.url)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(a)))
}

pub async fn fetch_attachment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Attachment>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, Attachment>(
            "SELECT * FROM attachments WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(att) = opt {
        Ok((StatusCode::OK, Json(att)))
    } else {
        Err((StatusCode::NOT_FOUND, "Attachment not found".to_string()))
    }
}

pub async fn remove_attachment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM attachments WHERE id = $1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
