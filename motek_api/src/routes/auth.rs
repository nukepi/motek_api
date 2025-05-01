use crate::database::token::create_jwt;
use crate::models::user::User;
use crate::state::AppState;
use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    routing::post,
};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginPayload {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

// pomocniczo
async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    // 1) duplikat?
    if sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE email = $1 LIMIT 1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .is_some()
    {
        return Err((StatusCode::CONFLICT, "Email already exists".to_string()));
    }

    // 2) hash hasła
    let password_hash = hash(&payload.password, 4)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    // 3) insert
    sqlx::query_as::<_, User>("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *")
        .bind(&payload.email)
        .bind(&password_hash)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json("User registered".to_string())))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    let user = get_user_by_email(&state.pool, &payload.email)
        .await
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))?;

    if verify(&payload.password, &user.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    {
        let secret = state
            .config
            .jwt_secret
            .as_deref()
            .expect("Missing JWT secret");
        let token = create_jwt(&user.email, secret);
        Ok((StatusCode::OK, Json(AuthResponse { token })))
    } else {
        Err((StatusCode::UNAUTHORIZED, "Invalid credentials".to_string()))
    }
}
