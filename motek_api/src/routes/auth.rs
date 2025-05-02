use crate::{
    database::token::revoke_all_refresh_tokens_for_user,
    database::token::{create_jwt, create_refresh_token, get_refresh_token, revoke_refresh_token},
    models::user::User,
    state::AppState,
    utils::extractors::AuthUser,
    utils::validators::{validate_email, validate_password},
};

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

/// Payload for user registration.
#[derive(Deserialize)]
pub struct RegisterPayload {
    pub email: String,
    pub password: String,
}

/// Payload for login.
#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
    pub platform: String, // "web", "android", "ios"
}

/// Response for login – returns both JWT and refresh token.
#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
}

/// Payload for refreshing JWT.
#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
    pub platform: String,
}

/// Response for refreshing JWT.
#[derive(Serialize)]
pub struct RefreshResponse {
    pub token: String,
}

/// Payload for logout.
#[derive(Deserialize)]
pub struct LogoutRequest {
    pub refresh_token: String,
}

/// Returns a router for authentication endpoints.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/refresh", post(refresh_jwt))
        .route("/logout", post(logout))
        .route("/logout_all", post(logout_all))
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

    // Validate password and email
    validate_password(&payload.password).map_err(|msg| (StatusCode::BAD_REQUEST, msg))?;
    validate_email(&payload.email).map_err(|msg| (StatusCode::BAD_REQUEST, msg))?;

    // 1) Check for duplicate email
    if sqlx::query_scalar::<_, Uuid>("SELECT id FROM users WHERE email = $1 LIMIT 1")
        .bind(&payload.email)
        .fetch_optional(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error during registration: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
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
    let password_hash = hash(&payload.password, 12).map_err(|e| {
        error!("Hashing error: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Database error".to_string(),
        )
    })?;

    // 3) Insert user
    sqlx::query_as::<_, User>("INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *")
        .bind(&payload.email)
        .bind(&password_hash)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| {
            error!("DB error during user insert: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            )
        })?;

    info!(
        "User {} registered successfully from IP {}",
        &payload.email, ip
    );

    Ok((StatusCode::CREATED, Json("User registered".to_string())))
}

/// Login and obtain a JWT token and refresh token.
/// Checks password and platform, returns both tokens if successful.
pub async fn login(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!(
        "Login attempt for email: {}, ip: {}",
        &payload.email,
        addr.ip()
    );
    let ip = addr.ip();
    let allowed = state.login_limiter.check_and_update(ip).await;
    if !allowed {
        info!("Login rate limit exceeded for IP: {}", ip);
        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            format!(
                "Login from this IP is allowed {} per hour",
                state.login_limiter.per_hour
            ),
        ));
    }
    let user = match get_user_by_email(&state.pool, &payload.email).await {
        Ok(user) => user,
        Err(sqlx::Error::RowNotFound) => {
            info!("Login failed: user not found for email {}", &payload.email);
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid email or password".to_string(),
            ));
        }
        Err(e) => {
            error!("DB error during login for {}: {}", &payload.email, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Database error".to_string(),
            ));
        }
    };

    if !verify(&payload.password, &user.password).unwrap_or(false) {
        info!("Login failed: invalid password for {}", &payload.email);
        return Err((
            StatusCode::UNAUTHORIZED,
            "Invalid email or password".to_string(),
        ));
    }

    let jwt_secret = state.config.jwt_secret.as_deref().unwrap_or("sekret_dev");
    let token = match create_jwt(&user.id.to_string(), &payload.platform, jwt_secret) {
        Ok(t) => t,
        Err(e) => {
            error!("JWT creation error for {}: {}", &payload.email, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "JWT error".to_string()));
        }
    };

    // Generate refresh token valid for 30 days
    let refresh_token = match create_refresh_token(&state.pool, user.id, 30).await {
        Ok(rt) => rt.token,
        Err(e) => {
            error!("Refresh token creation error for {}: {}", &payload.email, e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Refresh token error".to_string(),
            ));
        }
    };

    info!(
        "User {} logged in successfully (platform: {})",
        &payload.email, &payload.platform
    );
    Ok(Json(LoginResponse {
        token,
        refresh_token,
    }))
}

/// Refresh JWT using a valid refresh token.
/// Returns a new JWT if the refresh token is valid and not expired/revoked.
pub async fn refresh_jwt(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    info!(
        "Refresh attempt with refresh_token={}...",
        &payload.refresh_token[..8]
    );
    let rec = match get_refresh_token(&state.pool, &payload.refresh_token).await {
        Ok(Some(rt)) => rt,
        _ => {
            error!("Invalid refresh token");
            return Err((
                StatusCode::UNAUTHORIZED,
                "Invalid refresh token".to_string(),
            ));
        }
    };

    if rec.revoked || rec.expires_at < chrono::Utc::now() {
        error!("Refresh token is expired or revoked");
        return Err((
            StatusCode::UNAUTHORIZED,
            "Refresh token expired or revoked".to_string(),
        ));
    }

    let jwt_secret = state.config.jwt_secret.as_deref().unwrap_or("sekret_dev");
    let token = match create_jwt(&rec.user_id.to_string(), &payload.platform, jwt_secret) {
        Ok(t) => t,
        Err(e) => {
            error!("JWT creation error: {}", e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "JWT error".to_string()));
        }
    };

    info!("Issued new JWT for user_id={}", rec.user_id);
    Ok(Json(RefreshResponse { token }))
}

/// Logout: revoke a refresh token.
/// After this, the refresh token cannot be used again.
pub async fn logout(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
    Json(payload): Json<LogoutRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    info!(
        "Logout attempt with refresh_token={}...",
        &payload.refresh_token[..8]
    );
    let belongs = match token_belongs_to_user(&state.pool, &payload.refresh_token, user_id).await {
        Ok(b) => b,
        Err(e) => return Err((StatusCode::INTERNAL_SERVER_ERROR, format!("DB error: {e}"))),
    };
    if !belongs {
        return Err((
            StatusCode::FORBIDDEN,
            "Token does not belong to user".to_string(),
        ));
    }
    match revoke_refresh_token(&state.pool, &payload.refresh_token).await {
        Ok(_) => {
            info!("Refresh token revoked successfully");
            Ok(StatusCode::NO_CONTENT)
        }
        Err(e) => {
            error!("Logout error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Logout error".to_string(),
            ))
        }
    }
}

pub async fn token_belongs_to_user(
    pool: &PgPool,
    refresh_token: &str,
    user_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let rec = sqlx::query_scalar!(
        "SELECT EXISTS(SELECT 1 FROM refresh_tokens WHERE token = $1 AND user_id = $2)",
        refresh_token,
        user_id
    )
    .fetch_one(pool)
    .await?;
    Ok(rec.unwrap_or(false))
}

/// Logout from all devices (revoke all refresh tokens for this user).
pub async fn logout_all(
    State(state): State<AppState>,
    AuthUser(user_id): AuthUser,
) -> Result<StatusCode, (StatusCode, String)> {
    match revoke_all_refresh_tokens_for_user(&state.pool, user_id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => {
            error!("Logout all error: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                "Logout all error".to_string(),
            ))
        }
    }
}
