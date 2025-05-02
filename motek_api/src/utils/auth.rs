use crate::{database::token::verify_jwt, models::user::User, state::AppState};
use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use sqlx::query_as;
use tracing::{error, info};
use uuid::Uuid;

/// Authentication middleware for Axum.
/// Checks for a Bearer JWT token, verifies it, fetches the user, and inserts user_id into request extensions.
/// Logs all important steps and errors.
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    // 1) Extract Bearer token from Authorization header
    let token = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");

    if token.is_empty() {
        info!("Missing Bearer token in Authorization header");
        return Err((StatusCode::UNAUTHORIZED, "Missing Bearer token"));
    }

    // 2) Verify JWT token
    let secret = state
        .config
        .jwt_secret
        .as_deref()
        .expect("Missing jwt_secret in config");

    // After JWT verification
    let data = match verify_jwt(token, secret) {
        Ok(d) => d,
        Err(e) => {
            error!("Invalid JWT token: {}", e);
            return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
        }
    };

    // Parse UUID from JWT sub
    let user_id = uuid::Uuid::parse_str(&data.sub).map_err(|e| {
        error!("Invalid UUID in JWT sub: {}", e);
        (StatusCode::UNAUTHORIZED, "Invalid user ID in token")
    })?;

    // Fetch user by id
    let user: User = match query_as::<_, User>("SELECT * FROM users WHERE id=$1")
        .bind(user_id)
        .fetch_optional(&state.pool)
        .await
    {
        Ok(Some(u)) => u,
        Ok(None) => {
            info!("User with id {} not found", user_id);
            return Err((StatusCode::UNAUTHORIZED, "User does not exist"));
        }
        Err(e) => {
            error!("Database error when fetching user {}: {}", user_id, e);
            return Err((StatusCode::INTERNAL_SERVER_ERROR, "Database error"));
        }
    };

    info!("Authenticated user_id={} email={}", user.id, user.email);

    // Insert user_id into request extensions for downstream extractors
    req.extensions_mut().insert::<Uuid>(user.id);

    // Continue to next handler/middleware
    Ok(next.run(req).await)
}
