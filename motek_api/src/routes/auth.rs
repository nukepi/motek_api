use crate::database::token::create_jwt;
use crate::models::user::User;
use crate::state::AppState;
use axum::{
    Router,
    extract::{ConnectInfo, Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};
use bcrypt::{hash, verify};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::net::SocketAddr;
use tracing::{error, info};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub platform: String, // "web", "android", "ios"
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

/// Returns a router for authentication endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
}

/// Helper to fetch user by email.
async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| sqlx::Error::RowNotFound)
}

/// Register a new user.
/// Registration is rate-limited by IP address.
pub async fn register(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<(StatusCode, Json<String>), (StatusCode, String)> {
    info!("Registration attempt from IP: {}", addr.ip());
    let ip = addr.ip();
    let allowed = state.register_limiter.check_and_update(ip).await;
    if !allowed {
        info!("Registration rate limit exceeded for IP: {}", ip);
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            "Registration from this IP is allowed once per hour".to_string(),
        ));
    }
    // 1) Check for duplicate email
    if sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE email = $1 LIMIT 1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error during registration: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?
        .is_some()
    {
        info!(
            "Registration failed: email {} already exists",
            &payload.email
        );
        return Err((StatusCode::CONFLICT, "Email already exists".to_string()));
    }

    // 2) Hash password
    let password_hash = hash(&payload.password, 4).map_err(|e| {
        error!("Hashing error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
    })?;

    // 3) Insert user
    sqlx::query_as::<_, User>("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *")
        .bind(&payload.email)
        .bind(&password_hash)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error during user insert: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
        })?;

    info!(
        "User {} registered successfully from IP {}",
        &payload.email, ip
    );

    Ok((StatusCode::CREATED, Json("User registered".to_string())))
}

/// Login and obtain a JWT token.
/// Checks password and platform, returns token if successful.
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    info!("Login attempt for email: {}", &payload.email);
    let user = match get_user_by_email(&state.pool, &payload.email).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            info!("Login failed: user not found for email {}", &payload.email);
            return (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response();
        }
        Err(e) => {
            error!("DB error during login for {}: {}", &payload.email, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "Database error").into_response();
        }
    };

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        info!("Login failed: invalid password for {}", &payload.email);
        return (StatusCode::UNAUTHORIZED, "Invalid email or password").into_response();
    }

    let jwt_secret = state.config.jwt_secret.as_deref().unwrap_or("sekret_dev");
    let token = match create_jwt(&user.id.to_string(), &payload.platform, jwt_secret) {
        Ok(t) => t,
        Err(e) => {
            error!("JWT creation error for {}: {}", &payload.email, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, "JWT error").into_response();
        }
    };

    info!(
        "User {} logged in successfully (platform: {})",
        &payload.email, &payload.platform
    );
    Json(LoginResponse { token }).into_response()
}
