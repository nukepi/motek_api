//server.rs

use axum::{Router, serve};
use tokio::net::TcpListener;
use sqlx::{Pool, Postgres, PgPool};
use crate::utils::config_loader::Config;
use crate::state::AppState;
use crate::routes::api;

pub async fn run() {
    let config: Config = Config::load();
    let pool: Pool<Postgres> = PgPool::connect(&config.database_url)
        .await
        .expect("Nie udało się połączyć z bazą");
    let state = AppState::new(pool, config);

    let app: Router = Router::new()
        // Jeżeli chcesz wszystkie endpointy pod /api
        .nest("/api", api::router())
        // Jeżeli chcesz root bez /api, to zamiast .nest użyj .merge(api::router())
        .with_state(state.clone());


    let addr = (state.config.server_address.as_str(), state.config.port);

    let listener: TcpListener = TcpListener::bind(addr)
        .await
        .expect("Nie można zbindować portu");
    println!("Serwer startuje na http://localhost:3000");

    // użyj Server::bind, bo identycznie w axum 0.7+
    serve(listener, app).await.unwrap();
}
