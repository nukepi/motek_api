use crate::models::user_settings::UserSettings;
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

/// Returns a router for user settings related endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list).post(create))
        .route("/{id}", get(get_one).put(update).delete(delete_one))
}

/// List all user settings for the authenticated user.
pub async fn list(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<(StatusCode, Json<Vec<UserSettings>>), (StatusCode, String)> {
    info!("User {} requested their user settings list", user_id);
    let rows = sqlx::query_as::<_, UserSettings>("SELECT * FROM user_settings WHERE user_id = $1")
        .bind(user_id)
        .fetch_all(&state.pool)
        .await
        .map_err(|e| {
            error!("Failed to fetch user settings for user {}: {}", user_id, e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;
    info!("User {} fetched {} user settings", user_id, rows.len());
    Ok((StatusCode::OK, Json(rows)))
}

#[derive(Deserialize)]
pub struct CreateUserSettings {
    pub user_id: Uuid,
    pub lang: String,
    pub theme: String,
    pub timezone: String,
    pub notifications_enabled: bool,
    pub default_sort: String,
    pub editor_mode: String,
}

/// Create new user settings record.
/// Note: Make sure user_id matches the authenticated user.
pub async fn create(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(p): Json<CreateUserSettings>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    if p.user_id != user_id {
        error!(
            "User {} tried to create user_settings for another user {}",
            user_id, p.user_id
        );
        return Err((
            StatusCode::FORBIDDEN,
            "You can only create your own settings".to_string(),
        ));
    }
    info!(
        "User {} is creating user settings: lang={}, theme={}",
        user_id, p.lang, p.theme
    );
    let us = sqlx::query_as::<_, UserSettings>(
        "INSERT INTO user_settings 
             (user_id,lang,theme,timezone,notifications_enabled,default_sort,editor_mode)
             VALUES ($1,$2,$3,$4,$5,$6,$7) RETURNING *",
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
    .map_err(|e| {
        error!("Failed to create user settings for user {}: {}", user_id, e);
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;
    info!(
        "User {} successfully created user settings id={}",
        user_id, us.id
    );
    Ok((StatusCode::CREATED, Json(us)))
}

/// Fetch a single user settings record by id.
/// Note: Should check if the user owns this settings record.
pub async fn get_one(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    info!("User {} is fetching user_settings id={}", user_id, id);
    let opt =
        sqlx::query_as::<_, UserSettings>("SELECT * FROM user_settings WHERE id=$1 AND user_id=$2")
            .bind(id)
            .bind(user_id)
            .fetch_optional(&state.pool)
            .await
            .map_err(|e| {
                error!(
                    "Failed to fetch user_settings {} for user {}: {}",
                    id, user_id, e
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            })?;
    if let Some(us) = opt {
        info!("User {} fetched user_settings id={}", user_id, id);
        Ok((StatusCode::OK, Json(us)))
    } else {
        info!(
            "User {} tried to fetch missing or unauthorized user_settings id={}",
            user_id, id
        );
        Err((StatusCode::NOT_FOUND, "Not found".to_string()))
    }
}

#[derive(Deserialize)]
pub struct UpdateUserSettings {
    pub lang: Option<String>,
    pub theme: Option<String>,
    pub timezone: Option<String>,
    pub notifications_enabled: Option<bool>,
    pub default_sort: Option<String>,
    pub editor_mode: Option<String>,
}

/// Update user settings by id.
/// Note: Should check if the user owns this settings record.
pub async fn update(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
    Json(p): Json<UpdateUserSettings>,
) -> Result<(StatusCode, Json<UserSettings>), (StatusCode, String)> {
    info!("User {} is updating user_settings id={}", user_id, id);
    let res = sqlx::query(
        r#"UPDATE user_settings SET
            lang                  = COALESCE($2, lang),
            theme                 = COALESCE($3, theme),
            timezone              = COALESCE($4, timezone),
            notifications_enabled = COALESCE($5, notifications_enabled),
            default_sort          = COALESCE($6, default_sort),
            editor_mode           = COALESCE($7, editor_mode)
          WHERE id = $1 AND user_id = $8"#,
    )
    .bind(id)
    .bind(p.lang)
    .bind(p.theme)
    .bind(p.timezone)
    .bind(p.notifications_enabled)
    .bind(p.default_sort)
    .bind(p.editor_mode)
    .bind(user_id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        error!(
            "Failed to update user_settings {} for user {}: {}",
            id, user_id, e
        );
        (StatusCode::BAD_REQUEST, "Invalid request".to_string())
    })?;

    if res.rows_affected() == 0 {
        info!(
            "User {} tried to update missing or unauthorized user_settings id={}",
            user_id, id
        );
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    }

    get_one(State(state), AuthUser(user_id), Path(id)).await
}

/// Delete user settings by id.
/// Note: Should check if the user owns this settings record.
pub async fn delete_one(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!("User {} is deleting user_settings id={}", user_id, id);
    let res = sqlx::query("DELETE FROM user_settings WHERE id=$1 AND user_id=$2")
        .bind(id)
        .bind(user_id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            error!(
                "Failed to delete user_settings {} for user {}: {}",
                id, user_id, e
            );
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;
    if res.rows_affected() == 0 {
        info!(
            "User {} tried to delete missing or unauthorized user_settings id={}",
            user_id, id
        );
        return Err((StatusCode::NOT_FOUND, "Not found".to_string()));
    }
    info!("User {} deleted user_settings id={}", user_id, id);
    Ok(StatusCode::NO_CONTENT)
}
