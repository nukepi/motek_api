// src/routes/notebooks.rs

use crate::{state::AppState, utils::extractors::AuthUser};
use axum::{
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
    Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::models::{notebook::Notebook, note::Note};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
        .nest(
            "/{id}/notes",
            Router::new().route("/", get(list_notes_in_notebook)),
        )
}

// --- HANDLERY ---

pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<Notebook>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, Notebook>(
        "SELECT * FROM notebooks WHERE user_id = $1 ORDER BY name",
    )
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateNotebook {
    pub name: String,
    pub parent_id: Option<Uuid>,
}

pub async fn create(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(p): Json<CreateNotebook>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    let nb = sqlx::query_as::<_, Notebook>(
        "INSERT INTO notebooks (user_id, name, parent_id) \
         VALUES ($1, $2, $3) RETURNING *",
    )
    .bind(user_id)
    .bind(&p.name)
    .bind(p.parent_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(nb)))
}

pub async fn get_one(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, Notebook>(
        "SELECT * FROM notebooks WHERE id = $1 AND user_id = $2",
    )
    .bind(id)
    .bind(user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if let Some(nb) = opt {
        Ok((StatusCode::OK, Json(nb)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateNotebook {
    pub name: Option<String>,
    pub parent_id: Option<Uuid>,
}

pub async fn update(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateNotebook>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    let result = sqlx::query(
        r#"UPDATE notebooks SET
              name      = COALESCE($2, name),
              parent_id = COALESCE($3, parent_id)
          WHERE id = $1 AND user_id = $4"#,
    )
    .bind(id)
    .bind(p.name)
    .bind(p.parent_id)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    }

    // Ponownie pobierz zaktualizowany rekord
    get_one(State(state), AuthUser(user_id), Path(id)).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    let result = sqlx::query("DELETE FROM notebooks WHERE id = $1 AND user_id = $2")
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    }
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_notes_in_notebook(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(nb_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<Note>>), (StatusCode, String)> {
    let notes = sqlx::query_as::<_, Note>(
        "SELECT * FROM notes WHERE notebook_id = $1 AND user_id = $2 ORDER BY updated_at DESC",
    )
    .bind(nb_id)
    .bind(user_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(notes)))
}
