mod database {
    pub mod token;
}
mod models {
    pub mod user;
}
mod routes {
    pub mod auth;
}

mod utils {
    pub mod config_loader;
}

mod state;

use axum::{Router, routing::post, serve};
use tokio::net::TcpListener;
use state::AppState;
use routes::auth::{register, login};
use sqlx::{postgres::PgPool, Pool, Postgres};
use utils::config_loader::Config;

#[tokio::main]
async fn main() {
    //load config file
    let config: Config = Config::load();
    // Pobierz URL do bazy z env
    let database_url: String = config.database_url;
    let jwt_secret = config.jwt_secret.as_deref().expect("JWT_SECRET must be set");
    // Połącz się z bazą
    let pool: Pool<Postgres> = PgPool::connect(&database_url).await.unwrap();
    // Utwórz AppState z pool i JWT
    let state: AppState = AppState::new(pool, jwt_secret);

    let app = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .with_state(state);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Serwer startuje na http://localhost:3000");

    serve(listener, app).await.unwrap();
}
