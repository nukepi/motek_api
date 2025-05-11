use serde::{Serialize, Deserialize};
use tracing::{info, error, debug, warn};
use crate::utils::token::{AuthTokens, UserInfo, set_tokens, set_user_info};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: String,
    pub refresh_token: String,
}

/// Attempts to log in a user with the given email and password.
pub async fn login(email: String, password: String) -> AuthResponse {
    info!("Login attempt for email: {}", email);

    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| {
        let default_url = "http://139.59.138.164:3000".to_string();
        info!("API_URL environment variable not set, using default: {}", default_url);
        default_url
    });

    let payload = serde_json::json!({
        "email": email.clone(),  // Klonujemy email, żeby użyć go później
        "password": password,
        "platform": "android"
    });

    let url = format!("{}/api/auth/login", api_url);
    info!("Sending login request to URL: {}", url);
    debug!("Login payload: {}", serde_json::to_string_pretty(&payload).unwrap_or_default());

    match client.post(&url).json(&payload).send().await {
        Ok(resp) => {
            let status = resp.status();
            info!("Login response status: {}", status);
            
            match resp.text().await {
                Ok(txt) => {
                    info!("Login response body: {}", txt);
                    
                    if status.is_success() {
                        // Próba parsowania JSON
                        match serde_json::from_str::<serde_json::Value>(&txt) {
                            Ok(json) => {
                                debug!("Parsed login JSON response: {}", serde_json::to_string_pretty(&json).unwrap_or_default());

                                let token = json.get("token").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let refresh_token = json.get("refresh_token").and_then(|v| v.as_str()).map(|s| s.to_string());
                                let user_id = json.get("user_id").and_then(|v| v.as_str()).map(|s| s.to_string());

                                if let (Some(token), Some(refresh_token)) = (token.clone(), refresh_token.clone()) {
                                    let tokens = AuthTokens {
                                        token: token.clone(),
                                        refresh_token: refresh_token.clone(),
                                    };
                                    
                                    // Zapisz tokeny
                                    set_tokens(tokens);
                                    
                                    // Zapisz informacje o użytkowniku
                                    if let Some(user_id) = user_id {
                                        let user_info = UserInfo {
                                            email: email.clone(),
                                            user_id,
                                        };
                                        set_user_info(user_info);
                                    }
                                    
                                    info!("Login successful, tokens and user info saved");
                                    
                                    AuthResponse {
                                        success: true,
                                        message: "Login successful!".to_string(),
                                        token,
                                        refresh_token,
                                    }
                                } else {
                                    warn!("Missing tokens in login response JSON: {:?}", json);
                                    AuthResponse {
                                        success: false,
                                        message: "Missing tokens in server response".to_string(),
                                        token: "".to_string(),
                                        refresh_token: "".to_string(),
                                    }
                                }
                            }
                            Err(e) => {
                                error!("Failed to parse login JSON response: {}", e);
                                error!("Raw response was: {}", txt);
                                AuthResponse {
                                    success: false,
                                    message: format!("Failed to parse server response: {}", e),
                                    token: "".to_string(),
                                    refresh_token: "".to_string(),
                                }
                            }
                        }
                    } else {
                        error!("Login failed with status: {} and body: {}", status, txt);
                        AuthResponse {
                            success: false,
                            message: format!("Error: {} ({})", txt, status),
                            token: "".to_string(),
                            refresh_token: "".to_string(),
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read login response text: {}", e);
                    AuthResponse {
                        success: false,
                        message: format!("Failed to read server response: {}", e),
                        token: "".to_string(),
                        refresh_token: "".to_string(),
                    }
                }
            }
        }
        Err(e) => {
            error!("Connection error during login: {}", e);
            AuthResponse {
                success: false,
                message: format!("Connection error: {}", e),
                token: "".to_string(),
                refresh_token: "".to_string(),
            }
        }
    }
}


pub async fn register(email: String, password: String) -> AuthResponse {
    info!("Register attempt for email: {}", email);

    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| {
        let default_url = "http://139.59.138.164:3000".to_string();
        info!("API_URL environment variable not set, using default: {}", default_url);
        default_url
    });

    let payload = serde_json::json!({
        "email": email,
        "password": password,
    });

    let url = format!("{}/api/auth/register", api_url);
    info!("Sending register request to URL: {}", url);
    debug!("Register payload: {}", payload);

    match client.post(&url).json(&payload).send().await {
        Ok(resp) => {
            let status = resp.status();
            info!("Register response status: {}", status);
            
            match resp.text().await {
                Ok(txt) => {
                    info!("Register response body: {}", txt);
                    
                    if status.is_success() {
                        info!("Registration successful");
                        AuthResponse {
                            success: true,
                            message: "Registration successful!".to_string(),
                            token: "".to_string(),
                            refresh_token: "".to_string(),
                        }
                    } else {
                        error!("Registration failed with status: {} and body: {}", status, txt);
                        AuthResponse {
                            success: false,
                            message: format!("Error: {} ({})", txt, status),
                            token: "".to_string(),
                            refresh_token: "".to_string(),
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to read register response text: {}", e);
                    AuthResponse {
                        success: false,
                        message: format!("Failed to read server response: {}", e),
                        token: "".to_string(),
                        refresh_token: "".to_string(),
                    }
                }
            }
        }
        Err(e) => {
            error!("Connection error during registration: {}", e);
            AuthResponse {
                success: false,
                message: format!("Connection error: {}", e),
                token: "".to_string(),
                refresh_token: "".to_string(),
            }
        }
    }
}
