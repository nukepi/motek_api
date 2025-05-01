use axum::{
    Router,
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::shared_note::SharedNote;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",                     get(list).post(create))
        .route("/{note_id}/{user_id}",  get(get_one).put(update).delete(delete_one))
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<SharedNote>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, SharedNote>("SELECT * FROM shared_note")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateShare {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub role:    String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(p): Json<CreateShare>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    let now = Utc::now();
    let s = sqlx::query_as::<_, SharedNote>(
            "INSERT INTO shared_note (note_id,user_id,role,granted_at) \
             VALUES ($1,$2,$3,$4) RETURNING *"
        )
        .bind(p.note_id)
        .bind(p.user_id)
        .bind(&p.role)
        .bind(now)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(s)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, SharedNote>(
            "SELECT * FROM shared_note WHERE note_id=$1 AND user_id=$2"
        )
        .bind(note_id)
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Some(s) = opt {
        Ok((StatusCode::OK, Json(s)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateShare {
    pub role: Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
    Json(p): Json<UpdateShare>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    sqlx::query(
        "UPDATE shared_note SET role = COALESCE($3, role) \
         WHERE note_id = $1 AND user_id = $2"
    )
    .bind(note_id)
    .bind(user_id)
    .bind(p.role)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    get_one(State(state), Path((note_id, user_id))).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query(
        "DELETE FROM shared_note WHERE note_id=$1 AND user_id=$2"
    )
    .bind(note_id)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
