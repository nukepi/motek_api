// utils/config_loader.rs

use serde::Deserialize;
use std::fs;
use std::env;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: Option<String>,
    pub server_address:String,
    pub port: u16,
}


impl Config {
    pub fn load() -> Self {
        // Pobierz środowisko z ENV lub domyślnie "dev"
        let env = env::var("RUN_ENV").unwrap_or_else(|_| "dev".to_string());
        let config_file = format!("config.{}.toml", env);

        let config_str = fs::read_to_string(&config_file)
            .unwrap_or_else(|_| panic!("Nie można wczytać pliku: {}", config_file));

        toml::from_str(&config_str)
            .unwrap_or_else(|_| panic!("Błąd parsowania pliku TOML: {}", config_file))
    }
}