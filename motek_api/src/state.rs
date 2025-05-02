//! Application state container.
//! Holds database connection pool, configuration, and IP registration limiter.

use crate::utils::ip_limiter::IpLimiter;
use crate::utils::config_loader::Config;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

/// Shared application state, passed to handlers and middleware.
#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: Arc<Config>,
    pub register_limiter: Arc<IpLimiter>,
    pub login_limiter: Arc<IpLimiter>,
}

impl AppState {
    /// Constructs a new AppState with a database pool, configuration, and IP limiter.
    pub fn new(pool: Pool<Postgres>, config: Config) -> Self {
        let register_per_hour = config.register_ip_limit_per_hour.unwrap_or(1);
        let login_per_hour = config.login_ip_limit_per_hour.unwrap_or(1);
        AppState {
            pool,
            register_limiter: Arc::new(IpLimiter::new(register_per_hour)),
            login_limiter: Arc::new(IpLimiter::new(login_per_hour)),
            config: Arc::new(config),
        }
    }
}
