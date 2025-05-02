// utils/config_loader.rs

use serde::Deserialize;
use std::env;
use std::fs;
use tracing::{error, info};

/// Application configuration loaded from TOML file.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: Option<String>,
    pub server_address: String,
    pub port: u16,
    pub register_ip_limit_per_hour: Option<u32>,
    pub login_ip_limit_per_hour: Option<u32>
}

impl Config {
    /// Loads configuration based on RUN_ENV environment variable (default: "dev").
    /// Panics if file cannot be read or parsed.
    pub fn load() -> Self {
        // Get environment from ENV or default to "dev"
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());
        let config_file = format!("config.{}.toml", env);

        info!("Loading config from file: {}", config_file);

        let config_str = fs::read_to_string(&config_file);
        let config_str = match config_str {
            Ok(s) => s,
            Err(e) => {
                error!("Failed to read config file {}: {}", config_file, e);
                panic!("Cannot read config file: {}", config_file);
            }
        };

        match toml::from_str(&config_str) {
            Ok(cfg) => {
                info!("Config loaded successfully for env: {}", env);
                cfg
            }
            Err(e) => {
                error!("Failed to parse TOML config file {}: {}", config_file, e);
                panic!("Error parsing TOML config file: {}", config_file);
            }
        }
    }
}
