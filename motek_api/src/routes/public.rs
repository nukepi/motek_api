use crate::state::AppState;
use axum::{Router, extract::ConnectInfo, routing::get};
use std::net::SocketAddr;
use tracing::info;

/// Returns a router for public endpoints.
pub fn router() -> Router<AppState> {
    Router::new().route("/ip", get(get_ip))
}

/// Returns the IP address of the client.
pub async fn get_ip(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> String {
    info!("Public IP check from {}", addr);
    format!("Your IP address is: {addr}")
}
