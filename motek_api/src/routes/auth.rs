use axum::{
    extract::{State, Json},
    response::IntoResponse,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify};
use sqlx::PgPool;

use crate::models::user::User;
use crate::state::AppState;
use crate::database::token::create_jwt;

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

// Funkcja pomocnicza do pobierania użytkownika po emailu
async fn get_user_by_email(pool: &PgPool, email: &str) -> Option<User> {
    sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_optional(pool)
        .await
        .ok()
        .flatten()
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // Sprawdź czy użytkownik już istnieje
    if get_user_by_email(&state.pool, &payload.email).await.is_some() {
        eprintln!("[register] Email already exists: {}", &payload.email);
        return (StatusCode::CONFLICT, Json("Email already exists")).into_response();
    }

    // Hashowanie hasła
    let password_hash = match hash(&payload.password, 4) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("[register] Hash error for email {}: {:?}", &payload.email, e);
            return (StatusCode::INTERNAL_SERVER_ERROR, Json("Hash error")).into_response();
        }
    };

    // Dodaj użytkownika do bazy (id generuje się automatycznie!)
    let result = sqlx::query_as::<_, User>(
        "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING *"
    )
    .bind(&payload.email)
    .bind(&password_hash)
    .fetch_one(&state.pool)
    .await;

    match result {
        Ok(_user) => {
            println!("[register] User registered: {}", &payload.email);
            (StatusCode::CREATED, Json("User registered")).into_response()
        },
        Err(e) => {
            eprintln!("[register] DB error for email {}: {:?}", &payload.email, e);
            (StatusCode::INTERNAL_SERVER_ERROR, Json("Database error")).into_response()
        }
    }
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    let user = match get_user_by_email(&state.pool, &payload.email).await {
        Some(u) => u,
        None => return (StatusCode::UNAUTHORIZED, Json("Invalid credentials")).into_response(),
    };

    match verify(&payload.password, &user.password) {
        Ok(true) => {
            let token = create_jwt(&user.email, &state.jwt_secret);
            (StatusCode::OK, Json(AuthResponse { token })).into_response()
        }
        _ => (StatusCode::UNAUTHORIZED, Json("Invalid credentials")).into_response(),
    }
}
