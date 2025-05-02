use crate::database::token::{Claims, verify_jwt};
use crate::state::AppState;
use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
};
use tracing::{error, info};

/// Extractor for JWT claims from the Authorization header.
/// Validates the JWT and makes claims available for handlers.
pub struct AuthClaims(pub Claims);

impl FromRequestParts<AppState> for AuthClaims {
    type Rejection = (StatusCode, &'static str);

    /// Extracts and verifies JWT claims from the Authorization header.
    /// Logs extraction and validation results.
    fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> impl std::future::Future<Output = Result<Self, Self::Rejection>> + Send {
        async move {
            let jwt_secret = state
                .config
                .jwt_secret
                .as_deref()
                .ok_or((StatusCode::UNAUTHORIZED, "Missing JWT secret"))?;

            let auth_header = parts
                .headers
                .get("authorization")
                .and_then(|h| h.to_str().ok())
                .ok_or((StatusCode::UNAUTHORIZED, "Missing Authorization header"))?;

            let token = auth_header
                .strip_prefix("Bearer ")
                .ok_or((StatusCode::UNAUTHORIZED, "Wrong token format"))?;

            let claims = match verify_jwt(token, jwt_secret) {
                Ok(c) => {
                    info!("JWT successfully verified for subject={}", c.sub);
                    c
                }
                Err(e) => {
                    error!("JWT verification failed: {}", e);
                    return Err((StatusCode::UNAUTHORIZED, "Invalid token"));
                }
            };

            Ok(AuthClaims(claims))
        }
    }
}
