//! JWT token creation and verification logic.

use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use tracing::{error, info};

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
