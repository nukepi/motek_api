use std::sync::RwLock;
use once_cell::sync::Lazy;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)] 
pub struct AuthTokens {
    pub token: String,
    pub refresh_token: String,
}

pub static TOKENS: Lazy<RwLock<Option<AuthTokens>>> = Lazy::new(|| RwLock::new(None));
