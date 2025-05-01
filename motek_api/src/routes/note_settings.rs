use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::note_settings::NoteSettings;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<NoteSettings>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, NoteSettings>("SELECT * FROM note_settings")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateNoteSettings {
    pub note_id: Uuid,
    pub color: String,
    pub font: String,
    pub view_mode: String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(p): Json<CreateNoteSettings>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
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
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(ns)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, NoteSettings>("SELECT * FROM note_settings WHERE id=$1")
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(ns) = opt {
        Ok((StatusCode::OK, Json(ns)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateNoteSettings {
    pub color: Option<String>,
    pub font: Option<String>,
    pub view_mode: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateNoteSettings>,
) -> Result<(StatusCode, Json<NoteSettings>), (StatusCode, String)> {
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
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    get_one(State(state), Path(id)).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM note_settings WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
