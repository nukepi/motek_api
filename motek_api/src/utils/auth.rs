use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use crate::{database::token::verify_jwt, models::user::User, state::AppState};
use sqlx::query_as;
use uuid::Uuid;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, &'static str)> {
    // 1) Bearer token
    let token = req.headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .unwrap_or("");

    // 2) weryfikacja JWT
    let secret = state.config.jwt_secret
        .as_deref()
        .expect("Brak jwt_secret w configu");
    let data = verify_jwt(token, secret)
        .ok_or((StatusCode::UNAUTHORIZED, "Nieprawidłowy token"))?;

    // 3) pobranie usera
    let user: User = query_as::<_, User>("SELECT * FROM users WHERE email=$1")
        .bind(&data.claims.sub)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_e| (StatusCode::INTERNAL_SERVER_ERROR, "Błąd bazy danych"))?
        .ok_or((StatusCode::UNAUTHORIZED, "Użytkownik nie istnieje"))?;

    // 4) wstaw user_id do extensions
    req.extensions_mut().insert::<Uuid>(user.id);

    // 5) kontynuuj
    Ok(next.run(req).await)
}
