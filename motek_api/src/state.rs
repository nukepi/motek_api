use std::sync::Arc;
use sqlx::Pool;
use sqlx::Postgres;
use crate::utils::config_loader::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: Pool<Postgres>,
    pub config: Arc<Config>
}

impl AppState {
    pub fn new(pool: Pool<Postgres>, config: Config) -> Self {
        AppState {
            pool,
            config: Arc::new(config),
        }
    }
}
