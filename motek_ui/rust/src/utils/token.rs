use tracing::{ info, debug, error, warn };
use once_cell::sync::Lazy;
use serde::{ Deserialize, Serialize };
use std::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthTokens {
    pub token: String,
    pub refresh_token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    pub email: String,
    pub user_id: String,
}

pub static AUTH_TOKENS: Lazy<RwLock<Option<AuthTokens>>> = Lazy::new(|| {
    debug!("Initializing AUTH_TOKENS storage");
    // Próba wczytania tokenów
    let tokens = token_storage::load_tokens();
    if tokens.is_some() {
        info!("Loaded saved tokens from storage");
    }
    RwLock::new(tokens)
});

pub static USER_INFO: Lazy<RwLock<Option<UserInfo>>> = Lazy::new(|| {
    debug!("Initializing USER_INFO storage");
    // Próba wczytania informacji o użytkowniku
    let user_info = token_storage::load_user_info();
    if user_info.is_some() {
        info!("Loaded saved user info from storage");
    }
    RwLock::new(user_info)
});

pub fn set_tokens(tokens: AuthTokens) {
    match AUTH_TOKENS.write() {
        Ok(mut lock) => {
            debug!("Setting new auth tokens");
            *lock = Some(tokens.clone());
            
            // Zapisz tokeny do pliku
            if let Err(e) = token_storage::save_tokens(&tokens) {
                error!("Failed to save tokens to storage: {}", e);
            } else {
                info!("Tokens saved to storage");
            }
            
            // Aktualizuj również tokeny w helpers.rs
            match crate::utils::helpers::TOKENS.write() {
                Ok(mut helpers_lock) => {
                    *helpers_lock = Some(crate::utils::helpers::TokenData {
                        token: tokens.token.clone(),
                        refresh_token: Some(tokens.refresh_token.clone()),
                        expiry: None, // Możesz dodać parsowanie JWT, aby uzyskać czas wygaśnięcia
                    });
                    debug!("Helpers token storage updated successfully");
                },
                Err(e) => {
                    error!("Failed to update helpers token storage: {}", e);
                }
            }
            
            info!("Auth tokens updated successfully");
        },
        Err(e) => {
            error!("Failed to acquire write lock for auth tokens: {}", e);
        }
    }
}

pub async fn refresh_token() -> Result<AuthTokens, String> {
    debug!("Attempting to refresh token");
    
    // Sprawdź, czy mamy zapisany refresh_token
    let refresh_token = match AUTH_TOKENS.read() {
        Ok(lock) => match &*lock {
            Some(tokens) => tokens.refresh_token.clone(),
            None => {
                warn!("No refresh token available");
                return Err("No refresh token available".to_string());
            }
        },
        Err(e) => {
            error!("Failed to read auth tokens: {}", e);
            return Err(format!("Failed to read auth tokens: {}", e));
        }
    };
    
    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| {
        let default_url = "http://139.59.138.164:3000".to_string();
        info!("API_URL environment variable not set, using default: {}", default_url);
        default_url
    });
    
    let payload = serde_json::json!({
        "refresh_token": refresh_token,
    });
    
    let url = format!("{}/api/auth/refresh", api_url);
    info!("Sending refresh token request to URL: {}", url);
    
    match client.post(&url).json(&payload).send().await {
        Ok(resp) => {
            let status = resp.status();
            info!("Refresh token response status: {}", status);
            
            match resp.text().await {
                Ok(txt) => {
                    info!("Refresh token response body: {}", txt);
                    
                    if status.is_success() {
                        // Próba parsowania JSON
                        match serde_json::from_str::<serde_json::Value>(&txt) {
                            Ok(json) => {
                                debug!("Parsed refresh token JSON response: {}", serde_json::to_string_pretty(&json).unwrap_or_default());
                                
                                let token = json.get("token").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let refresh_token = json.get("refresh_token").and_then(|v| v.as_str()).map(|s| s.to_string());
                                
                                if let (Some(token), Some(refresh_token)) = (token.clone(), refresh_token.clone()) {
                                    let tokens = AuthTokens {
                                        token: token.clone(),
                                        refresh_token: refresh_token.clone(),
                                    };
                                    
                                    // Zapisz nowe tokeny
                                    set_tokens(tokens.clone());
                                    
                                    info!("Token refreshed successfully");
                                    
                                    return Ok(tokens);
                                } else {
                                    warn!("Missing tokens in refresh token response JSON: {:?}", json);
                                    return Err("Missing tokens in server response".to_string());
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse refresh token JSON response: {}", e);
                                error!("Raw response was: {}", txt);
                                return Err(format!("Failed to parse server response: {}", e));
                            }
                        }
                    } else {
                        error!("Refresh token failed with status: {} and body: {}", status, txt);
                        return Err(format!("Error: {} ({})", txt, status));
                    }
                }
                Err(e) => {
                    error!("Failed to read refresh token response text: {}", e);
                    return Err(format!("Failed to read server response: {}", e));
                }
            }
        }
        Err(e) => {
            error!("Connection error during refresh token: {}", e);
            return Err(format!("Connection error: {}", e));
        }
    }
}


pub fn clear_tokens() {
    match AUTH_TOKENS.write() {
        Ok(mut lock) => {
            debug!("Clearing auth tokens");
            *lock = None;

            // Usuń tokeny z pliku
            if let Err(e) = token_storage::delete_tokens() {
                error!("Failed to delete tokens from storage: {}", e);
            } else {
                info!("Tokens removed from storage");
            }

            info!("Auth tokens cleared successfully");
        }
        Err(e) => {
            error!("Failed to acquire write lock for auth tokens: {}", e);
        }
    }
}

pub fn get_tokens() -> Option<AuthTokens> {
    match AUTH_TOKENS.read() {
        Ok(lock) => { lock.clone() }
        Err(e) => {
            error!("Failed to read auth tokens: {}", e);
            None
        }
    }
}

pub fn has_tokens() -> bool {
    match AUTH_TOKENS.read() {
        Ok(lock) => { lock.is_some() }
        Err(e) => {
            error!("Failed to read auth tokens: {}", e);
            false
        }
    }
}

pub fn set_user_info(user_info: UserInfo) {
    match USER_INFO.write() {
        Ok(mut lock) => {
            debug!("Setting new user info");
            *lock = Some(user_info.clone());

            // Zapisz informacje o użytkowniku do pliku
            if let Err(e) = token_storage::save_user_info(&user_info) {
                error!("Failed to save user info to storage: {}", e);
            } else {
                info!("User info saved to storage");
            }

            info!("User info updated successfully");
        }
        Err(e) => {
            error!("Failed to acquire write lock for user info: {}", e);
        }
    }
}

pub fn clear_user_info() {
    match USER_INFO.write() {
        Ok(mut lock) => {
            debug!("Clearing user info");
            *lock = None;

            // Usuń informacje o użytkowniku z pliku
            if let Err(e) = token_storage::delete_user_info() {
                error!("Failed to delete user info from storage: {}", e);
            } else {
                info!("User info removed from storage");
            }

            info!("User info cleared successfully");
        }
        Err(e) => {
            error!("Failed to acquire write lock for user info: {}", e);
        }
    }
}

pub fn get_user_email() -> Option<String> {
    match USER_INFO.read() {
        Ok(lock) => {
            if let Some(user_info) = &*lock { Some(user_info.email.clone()) } else { None }
        }
        Err(e) => {
            error!("Failed to read user info: {}", e);
            None
        }
    }
}

pub fn get_user_id() -> Option<String> {
    match USER_INFO.read() {
        Ok(lock) => {
            if let Some(user_info) = &*lock { Some(user_info.user_id.clone()) } else { None }
        }
        Err(e) => {
            error!("Failed to read user info: {}", e);
            None
        }
    }
}

// Funkcja do wylogowania - czyści zarówno tokeny jak i informacje o użytkowniku
pub fn logout() {
    clear_tokens();
    clear_user_info();
    info!("User logged out successfully");
}

pub fn initialize_tokens() {
    debug!("Initializing tokens from storage");
    
    // Próba wczytania tokenów z pliku
    let tokens = token_storage::load_tokens();
    if let Some(tokens) = tokens {
        info!("Loaded tokens from storage, updating in-memory storage");
        match AUTH_TOKENS.write() {
            Ok(mut lock) => {
                *lock = Some(tokens);
                info!("In-memory token storage updated successfully");
            },
            Err(e) => {
                error!("Failed to update in-memory token storage: {}", e);
            }
        }
    } else {
        debug!("No saved tokens found in storage");
    }
    
    // Próba wczytania informacji o użytkowniku z pliku
    let user_info = token_storage::load_user_info();
    if let Some(user_info) = user_info {
        info!("Loaded user info from storage, updating in-memory storage");
        match USER_INFO.write() {
            Ok(mut lock) => {
                *lock = Some(user_info);
                info!("In-memory user info storage updated successfully");
            },
            Err(e) => {
                error!("Failed to update in-memory user info storage: {}", e);
            }
        }
    } else {
        debug!("No saved user info found in storage");
    }
}


pub mod token_storage {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn token_file_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| {
            debug!("Could not determine home directory, using current directory");
            PathBuf::from(".")
        });
        let path = home.join(".motek_tokens.json");
        debug!("Token file path: {:?}", path);
        path
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

    pub fn save_tokens(tokens: &AuthTokens) -> std::io::Result<()> {
        let path = token_file_path();
        debug!("Saving tokens to {:?}", path);

        match serde_json::to_string(tokens) {
            Ok(json) => {
                match fs::write(&path, &json) {
                    Ok(_) => {
                        info!("Tokens successfully saved to {:?}", path);
                        Ok(())
                    }
                    Err(e) => {
                        error!("Failed to write tokens to {:?}: {}", path, e);
                        Err(e)
                    }
                }
            }
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
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(tokens) => {
                        info!("Successfully loaded tokens from {:?}", path);
                        Some(tokens)
                    }
                    Err(e) => {
                        error!("Failed to deserialize tokens from {:?}: {}", path, e);
                        None
                    }
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    error!("Failed to read tokens from {:?}: {}", path, e);
                } else {
                    debug!("No token file found at {:?}", path);
                }
                None
            }
        }
    }

    pub fn delete_tokens() -> std::io::Result<()> {
        let path = token_file_path();
        debug!("Deleting tokens from {:?}", path);

        match fs::remove_file(&path) {
            Ok(_) => {
                info!("Successfully deleted tokens from {:?}", path);
                Ok(())
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    debug!("No token file to delete at {:?}", path);
                    Ok(())
                } else {
                    error!("Failed to delete tokens from {:?}: {}", path, e);
                    Err(e)
                }
            }
        }
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
                    }
                    Err(e) => {
                        error!("Failed to write user info to {:?}: {}", path, e);
                        Err(e)
                    }
                }
            }
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
            Ok(contents) => {
                match serde_json::from_str(&contents) {
                    Ok(user_info) => {
                        info!("Successfully loaded user info from {:?}", path);
                        Some(user_info)
                    }
                    Err(e) => {
                        error!("Failed to deserialize user info from {:?}: {}", path, e);
                        None
                    }
                }
            }
            Err(e) => {
                if e.kind() != std::io::ErrorKind::NotFound {
                    error!("Failed to read user info from {:?}: {}", path, e);
                } else {
                    debug!("No user info file found at {:?}", path);
                }
                None
            }
        }
    }

    pub fn delete_user_info() -> std::io::Result<()> {
        let path = user_info_file_path();
        debug!("Deleting user info from {:?}", path);

        match fs::remove_file(&path) {
            Ok(_) => {
                info!("Successfully deleted user info from {:?}", path);
                Ok(())
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    debug!("No user info file to delete at {:?}", path);
                    Ok(())
                } else {
                    error!("Failed to delete user info from {:?}: {}", path, e);
                    Err(e)
                }
            }
        }
    }
}
