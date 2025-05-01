use crate::utils::config_loader::Config;
use sqlx::Pool;
use sqlx::Postgres;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: Arc<Config>,
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, config: Config) -> Self {
        AppState {
            pool,
            config: Arc::new(config),
        }
    }
}
