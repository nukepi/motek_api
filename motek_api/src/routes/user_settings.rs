use axum::{
    Router,
    extract::{State, Path, Json},
    http::StatusCode,
    routing::get,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::state::AppState;
use crate::models::user_settings::UserSettings;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/",    get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<UserSettings>>), (StatusCode, String)> {
    let rows = sqlx::query_as::<_, UserSettings>("SELECT * FROM user_settings")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateUserSettings {
    pub user_id:               Uuid,
    pub lang:                  String,
    pub theme:                 String,
    pub timezone:              String,
    pub notifications_enabled: bool,
    pub default_sort:          String,
    pub editor_mode:           String,
}

pub async fn create(
    State(state): State<AppState>,
    Json(p): Json<CreateUserSettings>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    let us = sqlx::query_as::<_, UserSettings>(
            "INSERT INTO user_settings 
             (user_id,lang,theme,timezone,notifications_enabled,default_sort,editor_mode)
             VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *"
        )
        .bind(p.user_id)
        .bind(&p.lang)
        .bind(&p.theme)
        .bind(&p.timezone)
        .bind(p.notifications_enabled)
        .bind(&p.default_sort)
        .bind(&p.editor_mode)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok((StatusCode::CREATED, Json(us)))
}

pub async fn get_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    let opt = sqlx::query_as::<_, UserSettings>(
            "SELECT * FROM user_settings WHERE id=$1"
        )
        .bind(id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    if let Some(us) = opt {
        Ok((StatusCode::OK, Json(us)))
    } else {
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateUserSettings {
    pub lang:                  Option<String>,
    pub theme:                 Option<String>,
    pub timezone:              Option<String>,
    pub notifications_enabled: Option<bool>,
    pub default_sort:          Option<String>,
    pub editor_mode:           Option<String>,
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(p):    Json<UpdateUserSettings>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    sqlx::query(
        r#"UPDATE user_settings SET
            lang                  = COALESCE($2, lang),
            theme                 = COALESCE($3, theme),
            timezone              = COALESCE($4, timezone),
            notifications_enabled = COALESCE($5, notifications_enabled),
            default_sort          = COALESCE($6, default_sort),
            editor_mode           = COALESCE($7, editor_mode)
          WHERE id = $1"#
    )
    .bind(id)
    .bind(p.lang)
    .bind(p.theme)
    .bind(p.timezone)
    .bind(p.notifications_enabled)
    .bind(p.default_sort)
    .bind(p.editor_mode)
    .execute(&state.pool)
    .await
    .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    get_one(State(state), Path(id)).await
}

pub async fn delete_one(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    sqlx::query("DELETE FROM user_settings WHERE id=$1")
        .bind(id)
        .execute(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(StatusCode::NO_CONTENT)
}
