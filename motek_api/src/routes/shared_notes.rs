use crate::models::shared_note::SharedNote;
use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    Router,
    extract::{Json, Path, State},
    http::StatusCode,
    routing::get,
};
use chrono::Utc;
use serde::Deserialize;
use tracing::{error, info};
use uuid::Uuid;

/// Returns a router for shared notes endpoints.
pub fn router() -> Router<AppState> {
    Router::new().route("/", get(list).post(create)).route(
        "/{note_id}/{user_id}",
        get(get_one).put(update).delete(delete_one),
    )
}

/// List all shared notes for a user.
pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<SharedNote>>), (StatusCode, String)> {
    info!("User {} requested shared notes list", user_id);
    let rows = sqlx::query_as::<_, SharedNote>("SELECT * FROM shared_note WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error fetching shared notes for user {}: {}", user_id, e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateShare {
    pub note_id: Uuid,
    pub user_id: Uuid,
    pub role: String,
}

/// Create a shared note entry.
pub async fn create(
    State(state): State<AppState>,
    Json(p): Json<CreateShare>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    let now = Utc::now();
    info!(
        "Creating shared note for note_id {} and user_id {}",
        p.note_id, p.user_id
    );
    let s = sqlx::query_as::<_, SharedNote>(
        "INSERT INTO shared_note (note_id,user_id,role,granted_at) \
             VALUES ($1,$2,$3,$4) RETURNING *",
    )
    .bind(p.note_id)
    .bind(p.user_id)
    .bind(&p.role)
    .bind(now)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error creating shared note: {}", e);
        (StatusCode::BAD_REQUEST, e.to_string())
    })?;
    Ok((StatusCode::CREATED, Json(s)))
}

/// Get a shared note entry.
pub async fn get_one(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    info!(
        "Fetching shared note for note_id {} and user_id {}",
        note_id, user_id
    );
    let opt = sqlx::query_as::<_, SharedNote>(
        "SELECT * FROM shared_note WHERE note_id=$1 AND user_id=$2",
    )
    .bind(note_id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error fetching shared note: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;
    if let Some(s) = opt {
        Ok((StatusCode::OK, Json(s)))
    } else {
        info!(
            "Shared note not found for note_id {} and user_id {}",
            note_id, user_id
        );
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateShare {
    pub role: Option<String>,
}

/// Update a shared note entry.
pub async fn update(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
    Json(p): Json<UpdateShare>,
) -> Result<(StatusCode, Json<SharedNote>), (StatusCode, String)> {
    info!(
        "Updating shared note for note_id {} and user_id {}",
        note_id, user_id
    );
    sqlx::query(
        "UPDATE shared_note SET role = COALESCE($3, role) \
         WHERE note_id = $1 AND user_id = $2",
    )
    .bind(note_id)
    .bind(user_id)
    .bind(p.role)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!("DB error updating shared note: {}", e);
        (StatusCode::BAD_REQUEST, e.to_string())
    })?;

    get_one(State(state), Path((note_id, user_id))).await
}

/// Delete a shared note entry.
pub async fn delete_one(
    State(state): State<AppState>,
    Path((note_id, user_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!(
        "Deleting shared note for note_id {} and user_id {}",
        note_id, user_id
    );
    sqlx::query("DELETE FROM shared_note WHERE note_id=$1 AND user_id=$2")
        .bind(note_id)
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error deleting shared note: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;
    Ok(StatusCode::NO_CONTENT)
}
