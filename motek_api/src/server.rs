//! Server setup and runner.
//! Combines public and protected routes, applies middleware, and starts the HTTP server.

use crate::routes::public;
use crate::utils::config_loader::Config;
use crate::{
    routes::{api, auth},
    state::AppState,
    utils::auth::auth_middleware,
};
use axum::{Router, middleware};
use axum_server::Server;
use sqlx::PgPool;
use std::net::{SocketAddr, TcpListener};
use tower_http::trace::TraceLayer;
use tracing::info;

/// Runs the HTTP server.
/// Loads configuration, connects to the database, sets up state and routes, and starts listening.
pub async fn run() -> anyhow::Result<()> {
    // Load application configuration.
    let config = Config::load();
    info!("Configuration loaded");

    // Connect to the PostgreSQL database.
    let pool = PgPool::connect(&config.database_url).await?;
    info!("Connected to PostgreSQL");

    // Initialize application state.
    let state = AppState::new(pool, config);

    // Set up public endpoints.
    let api_public = Router::new()
        .nest("/api/auth", auth::router())
        .nest("/api/public", public::router())
        .with_state(state.clone());

    // Set up protected endpoints (all /api/* except /api/auth).
    let api_protected = Router::new()
        .nest("/api", api::router())
        .layer(TraceLayer::new_for_http())
        .layer(middleware::from_fn_with_state(
            state.clone(),
            auth_middleware,
        ))
        .with_state(state.clone());

    // Merge all routers into the main app.
    let app = Router::new().merge(api_public).merge(api_protected);

    // Bind TCP listener to configured address and port.
    let listener = TcpListener::bind((&*state.config.server_address, state.config.port))?;
    info!(
        "Server is online at http://{}:{}",
        state.config.server_address, state.config.port
    );
    println!("Serwer jest online!");
    println!(
        "Adres: http://{}:{}",
        state.config.server_address, state.config.port
    );

    // Start the Axum server.
    Server::from_tcp(listener)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await?;

    info!(
        "Server has shut down at http://{}:{}",
        state.config.server_address, state.config.port
    );

    Ok(())
}
