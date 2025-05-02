//! Application entry point.
//! Initializes logging and starts the server.

mod database;
mod models;
mod routes;
mod server;
mod state;
mod utils;

use tracing::{error, info};
use tracing_appender::rolling;
use tracing_subscriber::{fmt, prelude::*};

#[tokio::main]
async fn main() {
    // Set up daily rolling log file for tracing.
    let file_appender = rolling::daily("logs", "app.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let file_layer = fmt::layer().with_writer(non_blocking);

    // Log na konsolę (stdout)
    let console_layer = fmt::layer().with_writer(std::io::stdout);

    // Zbuduj subscriber z dwoma warstwami
    tracing_subscriber::registry()
        .with(console_layer)
        .with(file_layer)
        .init();

    info!("Starting Motek API server...");

    if let Err(e) = server::run().await {
        error!("Server startup error: {:?}", e);
        eprintln!("Server startup error: {:?}", e);
    }

    // Ensure log guard is not dropped early.
    let _ = _guard;
}
