use reqwest::Client;
use serde::de::DeserializeOwned;
use tracing::{info, debug, error, warn};
use std::sync::RwLock;
use once_cell::sync::Lazy;

const DEFAULT_API_URL: &str = "http://139.59.138.164:3000";
pub static TOKENS: Lazy<RwLock<Option<TokenData>>> = Lazy::new(|| RwLock::new(None));

#[derive(Clone, Debug)]
pub struct TokenData {
    pub token: String,
    pub refresh_token: Option<String>,
    pub expiry: Option<chrono::DateTime<chrono::Utc>>,
}

fn api_url() -> String {
    let url = std::env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string());
    debug!("Using API URL: {}", url);
    url
}

pub async fn authorized_post<T: DeserializeOwned>(
    endpoint: &str,
    payload: serde_json::Value,
) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    
    let token = match TOKENS.read() {
        Ok(lock) => match &*lock {
            Some(t) => t.token.clone(),
            None => {
                warn!("No token available for authorized_post request to {}", endpoint);
                return None;
            }
        },
        Err(e) => {
            error!("Failed to read token: {}", e);
            return None;
        }
    };

    let url = format!("{}{}", api_url, endpoint);
    info!("POST request to {} with payload: {}", url, payload);
    
    match client
        .post(&url)
        .bearer_auth(&token)
        .json(&payload)
        .send()
        .await {
            Ok(resp) => {
                let status = resp.status();
                info!("POST response status: {} for URL: {}", status, url);
                
                // Pobierz ciało odpowiedzi jako tekst
                match resp.text().await {
                    Ok(text) => {
                        info!("POST response body from {}: {}", url, text);
                        
                        // Próba parsowania tekstu do typu T
                        match serde_json::from_str::<T>(&text) {
                            Ok(data) => {
                                debug!("POST request to {} successfully parsed", url);
                                Some(data)
                            },
                            Err(e) => {
                                error!("Failed to parse POST response from {}: {}", url, e);
                                error!("Raw response was: {}", text);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to read response body from {}: {}", url, e);
                        None
                    }
                }
            },
            Err(e) => {
                error!("POST request to {} failed: {}", url, e);
                None
            }
        }
}


pub async fn authorized_get<T: DeserializeOwned>(endpoint: &str) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    
    // Upewnij się, że tokeny są dostępne
    if !ensure_tokens_available() {
        warn!("No token available for authorized_get request to {}", endpoint);
        return None;
    }
    
    let token = match TOKENS.read() {
        Ok(lock) => match &*lock {
            Some(t) => {
                let masked_token = if t.token.len() > 10 {
                    format!("{}...{}", &t.token[0..5], &t.token[t.token.len()-5..])
                } else {
                    "token too short to mask".to_string()
                };
                info!("Using token for request: {}", masked_token);
                t.token.clone()
            },
            None => {
                warn!("No token available for authorized_get request to {}", endpoint);
                return None;
            }
        },
        Err(e) => {
            error!("Failed to read token: {}", e);
            return None;
        }
    };

    let url = format!("{}{}", api_url, endpoint);
    info!("GET request to {}", url);
    
    match client
        .get(&url)
        .bearer_auth(&token)
        .send()
        .await {
            Ok(resp) => {
                let status = resp.status();
                info!("GET response status: {} for URL: {}", status, url);
                
                // Pobierz ciało odpowiedzi jako tekst
                match resp.text().await {
                    Ok(text) => {
                        info!("GET response body from {}: {}", url, text);
                        
                        // Próba parsowania tekstu do typu T
                        match serde_json::from_str::<T>(&text) {
                            Ok(data) => {
                                debug!("GET request to {} successfully parsed", url);
                                Some(data)
                            },
                            Err(e) => {
                                error!("Failed to parse GET response from {}: {}", url, e);
                                error!("Raw response was: {}", text);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to read response body from {}: {}", url, e);
                        None
                    }
                }
            },
            Err(e) => {
                error!("GET request to {} failed: {}", url, e);
                None
            }
        }
}



pub async fn authorized_put<T: DeserializeOwned>(
    endpoint: &str,
    payload: serde_json::Value,
) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    
    let token = match TOKENS.read() {
        Ok(lock) => match &*lock {
            Some(t) => t.token.clone(),
            None => {
                warn!("No token available for authorized_put request to {}", endpoint);
                return None;
            }
        },
        Err(e) => {
            error!("Failed to read token: {}", e);
            return None;
        }
    };

    let url = format!("{}{}", api_url, endpoint);
    info!("PUT request to {} with payload: {}", url, payload);
    
    match client
        .put(&url)
        .bearer_auth(&token)
        .json(&payload)
        .send()
        .await {
            Ok(resp) => {
                let status = resp.status();
                info!("PUT response status: {} for URL: {}", status, url);
                
                // Pobierz ciało odpowiedzi jako tekst
                match resp.text().await {
                    Ok(text) => {
                        info!("PUT response body from {}: {}", url, text);
                        
                        // Próba parsowania tekstu do typu T
                        match serde_json::from_str::<T>(&text) {
                            Ok(data) => {
                                debug!("PUT request to {} successfully parsed", url);
                                Some(data)
                            },
                            Err(e) => {
                                error!("Failed to parse PUT response from {}: {}", url, e);
                                error!("Raw response was: {}", text);
                                None
                            }
                        }
                    },
                    Err(e) => {
                        error!("Failed to read response body from {}: {}", url, e);
                        None
                    }
                }
            },
            Err(e) => {
                error!("PUT request to {} failed: {}", url, e);
                None
            }
        }
}


pub async fn authorized_delete(endpoint: &str) -> Option<bool> {
    let client = Client::new();
    let api_url = api_url();
    
    let token = match TOKENS.read() {
        Ok(lock) => match &*lock {
            Some(t) => t.token.clone(),
            None => {
                warn!("No token available for authorized_delete request to {}", endpoint);
                return None;
            }
        },
        Err(e) => {
            error!("Failed to read token: {}", e);
            return None;
        }
    };

    let url = format!("{}{}", api_url, endpoint);
    info!("DELETE request to {}", url);
    
    match client
        .delete(&url)
        .bearer_auth(&token)
        .send()
        .await {
            Ok(resp) => {
                let status = resp.status();
                info!("DELETE response status: {} for URL: {}", status, url);
                
                // Zapisz status przed wywołaniem text()
                let success = status.is_success();
                
                // Próba pobrania ciała odpowiedzi
                match resp.text().await {
                    Ok(text) => {
                        if !text.is_empty() {
                            info!("DELETE response body from {}: {}", url, text);
                        } else {
                            debug!("DELETE response body is empty from {}", url);
                        }
                    },
                    Err(e) => {
                        debug!("Could not read DELETE response body from {}: {}", url, e);
                    }
                }
                
                debug!("DELETE request to {} {}", url, if success { "successful" } else { "failed" });
                Some(success)
            },
            Err(e) => {
                error!("DELETE request to {} failed: {}", url, e);
                None
            }
        }
}

pub fn initialize_tokens_from_main_storage() {
    debug!("Initializing helpers token storage from main storage");
    
    // Pobierz tokeny z głównego magazynu
    if let Some(tokens) = crate::utils::token::get_tokens() {
        match TOKENS.write() {
            Ok(mut lock) => {
                *lock = Some(TokenData {
                    token: tokens.token,
                    refresh_token: Some(tokens.refresh_token),
                    expiry: None,
                });
                info!("Helpers token storage initialized from main storage");
            },
            Err(e) => {
                error!("Failed to update helpers token storage: {}", e);
            }
        }
    } else {
        debug!("No tokens found in main storage");
    }
}

pub fn ensure_tokens_available() -> bool {
    debug!("Ensuring tokens are available before API request");
    
    // Sprawdź, czy tokeny są dostępne w helpers.rs
    let tokens_available = match TOKENS.read() {
        Ok(lock) => lock.is_some(),
        Err(_) => false,
    };
    
    // Jeśli nie ma tokenów w helpers.rs, spróbuj je pobrać z głównego magazynu
    if !tokens_available {
        debug!("No tokens in helpers storage, trying to load from main storage");
        initialize_tokens_from_main_storage();
        
        // Sprawdź ponownie
        match TOKENS.read() {
            Ok(lock) => {
                let available = lock.is_some();
                if available {
                    info!("Successfully loaded tokens from main storage");
                } else {
                    warn!("No tokens available in any storage");
                }
                available
            },
            Err(e) => {
                error!("Failed to read helpers token storage: {}", e);
                false
            }
        }
    } else {
        true
    }
}


