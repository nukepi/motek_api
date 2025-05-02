//! Application state container.
//! Holds database connection pool, configuration, and IP registration limiter.

use crate::{utils::config_loader::Config, utils::ip_limiter::IpRegisterLimiter};
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

/// Shared application state, passed to handlers and middleware.
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: Arc<Config>,
    pub register_limiter: Arc<IpRegisterLimiter>,
}

impl AppState {
    /// Constructs a new AppState with a database pool, configuration, and IP limiter.
    pub fn new(pool: Pool<Postgres>, config: Config) -> Self {
        let per_hour = config.ip_limit_per_hour.unwrap_or(1);
        AppState {
            pool,
            register_limiter: Arc::new(IpRegisterLimiter::new(per_hour)),
            config: Arc::new(config),
        }
    }
}
