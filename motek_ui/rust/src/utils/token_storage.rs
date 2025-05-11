use std::fs;
use std::path::PathBuf;
use crate::utils::token::AuthTokens;
use crate::utils::token::UserInfo;
use tracing::{info, debug, error};

fn token_file_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| {
        debug!("Could not determine home directory, using current directory");
        PathBuf::from(".")
    });
    let path = home.join(".motek_tokens.json");
    debug!("Token file path: {:?}", path);
    path
}

pub fn save_tokens(tokens: &AuthTokens) -> std::io::Result<()> {
    let path = token_file_path();
    debug!("Saving tokens to {:?}", path);
    
    match serde_json::to_string(tokens) {
        Ok(json) => {
            match fs::write(&path, &json) {
                Ok(_) => {
                    info!("Tokens successfully saved to {:?}", path);
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to write tokens to {:?}: {}", path, e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            error!("Failed to serialize tokens: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}

pub fn load_tokens() -> Option<AuthTokens> {
    let path = token_file_path();
    debug!("Loading tokens from {:?}", path);
    
    match fs::read_to_string(&path) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(tokens) => {
                    info!("Tokens successfully loaded from {:?}", path);
                    Some(tokens)
                },
                Err(e) => {
                    error!("Failed to parse tokens from {:?}: {}", path, e);
                    None
                }
            }
        },
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                error!("Failed to read token file {:?}: {}", path, e);
            } else {
                debug!("Token file {:?} not found", path);
            }
            None
        }
    }
}

fn user_info_file_path() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| {
        debug!("Could not determine home directory, using current directory");
        PathBuf::from(".")
    });
    let path = home.join(".motek_user_info.json");
    debug!("User info file path: {:?}", path);
    path
}

pub fn save_user_info(user_info: &UserInfo) -> std::io::Result<()> {
    let path = user_info_file_path();
    debug!("Saving user info to {:?}", path);
    
    match serde_json::to_string(user_info) {
        Ok(json) => {
            match fs::write(&path, &json) {
                Ok(_) => {
                    info!("User info successfully saved to {:?}", path);
                    Ok(())
                },
                Err(e) => {
                    error!("Failed to write user info to {:?}: {}", path, e);
                    Err(e)
                }
            }
        },
        Err(e) => {
            error!("Failed to serialize user info: {}", e);
            Err(std::io::Error::new(std::io::ErrorKind::Other, e))
        }
    }
}

pub fn load_user_info() -> Option<UserInfo> {
    let path = user_info_file_path();
    debug!("Loading user info from {:?}", path);
    
    match fs::read_to_string(&path) {
        Ok(data) => {
            match serde_json::from_str(&data) {
                Ok(user_info) => {
                    info!("User info successfully loaded from {:?}", path);
                    Some(user_info)
                },
                Err(e) => {
                    error!("Failed to parse user info from {:?}: {}", path, e);
                    None
                }
            }
        },
        Err(e) => {
            if e.kind() != std::io::ErrorKind::NotFound {
                error!("Failed to read user info file {:?}: {}", path, e);
            } else {
                debug!("User info file {:?} not found", path);
            }
            None
        }
    }
}

pub fn delete_user_info() -> std::io::Result<()> {
    let path = user_info_file_path();
    debug!("Deleting user info from {:?}", path);
    
    if path.exists() {
        match fs::remove_file(&path) {
            Ok(_) => {
                info!("User info successfully deleted from {:?}", path);
                Ok(())
            },
            Err(e) => {
                error!("Failed to delete user info from {:?}: {}", path, e);
                Err(e)
            }
        }
    } else {
        debug!("User info file {:?} does not exist, nothing to delete", path);
        Ok(())
    }
}
