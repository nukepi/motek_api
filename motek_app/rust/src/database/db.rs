use sqlx::{PgPool, postgres::PgPoolOptions};
use crate::model::user::User;

pub async fn get_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Nie można połączyć się z bazą")
}

/* user registration */
pub async fn register_user(pool: &PgPool, username: &str, password: &str, email: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING id, username, password, email"
    )
    .bind(username)
    .bind(password)
    .bind(email)
    .fetch_one(pool)
    .await?;

    Ok(user)
}

/* user login */
pub async fn login_user(pool: &PgPool, username: &str, password: &str) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password, email FROM users WHERE username = $1 AND password = $2"
    )
    .bind(username)
    .bind(password)
    .fetch_one(pool)
    .await?;

    Ok(user)
}
