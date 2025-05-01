use axum::{middleware, Router};
use axum_server::Server;
use crate::{routes::{api, auth}, state::AppState, utils::auth::auth_middleware};
use tower_http::trace::TraceLayer;
use crate::utils::config_loader::Config;
use sqlx::PgPool;
use std::net::TcpListener;

pub async fn run() -> anyhow::Result<()> {
    let config = Config::load();
    let pool = PgPool::connect(&config.database_url).await?;
    let state = AppState::new(pool, config);

    // PUBLICZNE ENDPOINTY
    let api_public = Router::new()
        .nest("/api/auth", auth::router())
        .with_state(state.clone());

    // WSZYSTKO CO /api/* OPRÓCZ /api/auth JEST CHRONIONE
    let api_protected = Router::new()
        .nest("/api", api::router())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware))
        .with_state(state.clone());

    // POŁĄCZ WSZYSTKO
    let app = Router::new()
        .merge(api_public)
        .merge(api_protected);

    // Start serwera
    let listener = TcpListener::bind((&*state.config.server_address, state.config.port))?;

    println!("Serwer jest online!");
    println!("Adres: http://{}:{}", state.config.server_address, state.config.port);

    Server::from_tcp(listener)
        .serve(app.into_make_service())
        .await?;
        println!("Server is up at:{}:{}", state.config.server_address, state.config.port);

    Ok(())
}
