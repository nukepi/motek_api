use crate::models::note_settings::NoteSettings;
use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

/// Returns a router for note settings endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

/// List all note settings for notes owned by the user.
pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<NoteSettings>>), (StatusCode, String)> {
    info!("User {} requested note settings list", user_id);
    let rows = sqlx::query_as::<_, NoteSettings>(
        "SELECT ns.* FROM note_settings ns
         JOIN notes n ON ns.note_id = n.id
         WHERE n.user_id = $1",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        error!(
            "DB error fetching note settings for user {}: {}",
            user_id, e
        );
        (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
    })?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateNoteSettings {
    pub note_id: Uuid,
    pub color: String,
    pub font: String,
    pub view_mode: String,
}

/// Create note settings for a note.
/// Should check if the user owns the note.
pub async fn create(
    State(state): State<AppState>,
    Json(p): Json<CreateNoteSettings>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
    info!("Creating note settings for note_id {}", p.note_id);
    let ns = sqlx::query_as::<_, NoteSettings>(
        "INSERT INTO note_settings (note_id,color,font,view_mode) \
             VALUES ($1,$2,$3,$4) RETURNING *",
    )
    .bind(p.note_id)
    .bind(&p.color)
    .bind(&p.font)
    .bind(&p.view_mode)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!(
            "DB error creating note settings for note_id {}: {}",
            p.note_id, e
        );
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;
    Ok((StatusCode::CREATED, Json(ns)))
}

/// Get one note settings by id.
/// Should check if the user owns the note.
pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
    info!("Fetching note settings with id {}", id);
    let opt = sqlx::query_as::<_, NoteSettings>("SELECT * FROM note_settings WHERE id=$1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error fetching note settings {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
        })?;

    if let Some(ns) = opt {
        Ok((StatusCode::OK, Json(ns)))
    } else {
        info!("Note settings with id {} not found", id);
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateNoteSettings {
    pub color: Option<String>,
    pub font: Option<String>,
    pub view_mode: Option<String>,
}

/// Update note settings by id.
/// Should check if the user owns the note.
pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateNoteSettings>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
    info!("Updating note settings with id {}", id);
    sqlx::query(
        r#"UPDATE note_settings SET
            color     = COALESCE($2, color),
            font      = COALESCE($3, font),
            view_mode = COALESCE($4, view_mode)
          WHERE id = $1"#,
    )
    .bind(id)
    .bind(p.color)
    .bind(p.font)
    .bind(p.view_mode)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error updating note settings {}: {}", id, e);
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;

    get_one(State(state), Path(id)).await
}

/// Delete note settings by id.
/// Should check if the user owns the note.
pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("Deleting note settings with id {}", id);
    sqlx::query("DELETE FROM note_settings WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error deleting note settings {}: {}", id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Database error".to_string())
        })?;
    Ok(StatusCode::NO_CONTENT)
}
