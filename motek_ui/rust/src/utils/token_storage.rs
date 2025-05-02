use std::fs;
use std::path::PathBuf;
use crate::utils::token::AuthTokens;

fn token_file_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    home.join(".motek_tokens.json")
}

pub fn save_tokens(tokens: &AuthTokens) -> std::io::Result<()> {
    let json = serde_json::to_string(tokens)?;
    fs::write(token_file_path(), json)?;
    Ok(())
}

pub fn load_tokens() -> Option<AuthTokens> {
    let path = token_file_path();
    let data = fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}
