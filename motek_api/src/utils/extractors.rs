use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
};
use uuid::Uuid;

pub struct AuthUser(pub Uuid);

#[derive(Debug)]
pub enum AuthRejection {
    Missing,
}

impl IntoResponse for AuthRejection {
    fn into_response(self) -> Response {
        match self {
            AuthRejection::Missing => {
                (StatusCode::UNAUTHORIZED, "Brak uwierzytelnionego użytkownika").into_response()
            }
        }
    }
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = AuthRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // odczytujemy user_id z extensions
        parts
            .extensions
            .get::<Uuid>()
            .cloned()
            .map(AuthUser)
            .ok_or(AuthRejection::Missing)
    }
}
