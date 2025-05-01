use axum::{
    Router,
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::notebook::Notebook;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",    get(list_notebooks).post(create_notebook))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

pub async fn list_notebooks(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Notebook>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, Notebook>("SELECT * FROM notebooks")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateNotebook {
    pub name:      String,
    pub parent_id: Option<Uuid>,
}

pub async fn create_notebook(
    State(state): State<AppState>,
    Json(p): Json<CreateNotebook>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    let nb = sqlx::query_as::<_, Notebook>(
            "INSERT INTO notebooks (user_id,name,parent_id) \
             VALUES ($1,$2,$3) RETURNING *"
        )
        .bind(Uuid::new_v4()) // TODO: current_user_id
        .bind(&p.name)
        .bind(p.parent_id)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(nb)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, Notebook>("SELECT * FROM notebooks WHERE id=$1")
        .bind(id)
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
    pub name:      Option<String>,
    pub parent_id: Option<Uuid>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p):  Json<UpdateNotebook>,
) -> Result<(StatusCode, Json<Notebook>), (StatusCode, String)> {
    sqlx::query(
        r#"UPDATE notebooks SET
            name      = COALESCE($2, name),
            parent_id = COALESCE($3, parent_id)
          WHERE id = $1"#
    )
    .bind(id)
    .bind(p.name)
    .bind(p.parent_id)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    get_one(State(state), Path(id)).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM notebooks WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
