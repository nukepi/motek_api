//! JWT token creation and verification logic.

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::{error, info};
use rand::Rng;
use rand::distr::Alphanumeric;
use crate::models::refresh_token::RefreshToken;
use sqlx::PgPool;
use uuid::Uuid;

/// Generates a secure random refresh token string.
pub fn generate_refresh_token() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(64)
        .map(char::from)
        .collect()
}

/// Creates and stores a refresh token for a user in the database.
pub async fn create_refresh_token(
    pool: &PgPool,
    user_id: Uuid,
    expires_in_days: i64,
) -> Result<RefreshToken, sqlx::Error> {
    let token = generate_refresh_token();
    let expires_at = Utc::now() + Duration::days(expires_in_days);

    let rec = sqlx::query_as::<_, RefreshToken>(
        "INSERT INTO refresh_tokens (user_id, token, expires_at) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(user_id)
    .bind(&token)
    .bind(expires_at)
    .fetch_one(pool)
    .await?;

    info!("Created new refresh token for user_id={}", user_id);
    Ok(rec)
}

/// Fetches a refresh token by its string value.
pub async fn get_refresh_token(pool: &PgPool, token: &str) -> Result<Option<RefreshToken>, sqlx::Error> {
    let rec = sqlx::query_as::<_, RefreshToken>(
        "SELECT * FROM refresh_tokens WHERE token = $1"
    )
    .bind(token)
    .fetch_optional(pool)
    .await?;
    if rec.is_some() {
        info!("Fetched refresh token from DB");
    }
    Ok(rec)
}

/// Revokes a single refresh token (e.g., on logout).
pub async fn revoke_refresh_token(pool: &PgPool, token: &str) -> Result<(), sqlx::Error> {
    let _ = sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE token = $1")
        .bind(token)
        .execute(pool)
        .await?;
    info!("Revoked refresh token (token={}...)", &token[..8]);
    Ok(())
}


/// JWT Claims structure.
/// - `sub`: subject (usually user email or ID)
/// - `exp`: expiration timestamp (seconds since epoch)
/// - `platform`: platform string (e.g., "web", "mobile")
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub platform: String,
}

/// Creates a JWT for a given user and platform.
/// Returns the encoded JWT string.
pub fn create_jwt(
    username: &str,
    platform: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_string(),
        exp: expiration,
        platform: platform.to_string(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    );
    match &token {
        Ok(_) => info!("JWT created for user={} platform={}", username, platform),
        Err(e) => error!("JWT creation failed for user={}: {}", username, e),
    }
    token
}

/// Verifies a JWT and returns the claims if valid.
pub fn verify_jwt(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let result = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    );
    match &result {
        Ok(data) => info!("JWT verified for subject={}", data.claims.sub),
        Err(e) => error!("JWT verification failed: {}", e),
    }
    result.map(|data| data.claims)
}

/// Revokes all refresh tokens for a given user (logout from all devices).
pub async fn revoke_all_refresh_tokens_for_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("UPDATE refresh_tokens SET revoked = TRUE WHERE user_id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}

pub async fn cleanup_expired_refresh_tokens(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let res = sqlx::query("DELETE FROM refresh_tokens WHERE expires_at < NOW() OR revoked = TRUE")
        .execute(pool)
        .await?;
    Ok(res.rows_affected())
}
