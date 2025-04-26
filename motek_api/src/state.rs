use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(pool: PgPool, secret: &str) -> Self {
        AppState {
            pool,
            jwt_secret: secret.to_string(),
        }
    }
}
