use axum::{
    extract::FromRequestParts,
    http::{StatusCode, request::Parts},
    response::{IntoResponse, Response},
};
use tracing::info;
use uuid::Uuid;

/// Wrapper struct for authenticated user's UUID, extracted from request extensions.
pub struct AuthUser(pub Uuid);

/// Represents possible reasons for authentication extraction failure.
#[derive(Debug)]
pub enum AuthRejection {
    Missing,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        match self {
            AuthRejection::Missing => {
                (StatusCode::UNAUTHORIZED, "No authenticated user found").into_response()
            }
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    /// Extracts AuthUser from request extensions.
    /// Logs extraction result.
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let user_id = parts.extensions.get::<Uuid>().cloned();

        match user_id {
            Some(uid) => {
                info!("Extracted authenticated user_id={}", uid);
                Ok(AuthUser(uid))
            }
            None => {
                info!("No authenticated user found in request extensions");
                Err(AuthRejection::Missing)
            }
        }
    }
}
