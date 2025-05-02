use serde::{Serialize, Deserialize};
use tracing::{info, error, debug, warn};
use crate::utils::token::{TOKENS, AuthTokens};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: String,
    pub refresh_token: String,
}

/// Attempts to log in a user with the given email and password.
pub async fn login(email: String, password: String) -> AuthResponse {
    info!("Login called with email: {}", email);

    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://139.59.138.164:3000".to_string());

    let payload = serde_json::json!({
        "email": email,
        "password": password,
        "platform": "android"
    });

    let url = format!("{}/api/auth/login", api_url);

    debug!("Sending POST to {} with payload: {}", url, payload);

    let res = client.post(&url)
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(resp) => {
            debug!("Server response status: {}", resp.status());

            let status = resp.status();
            let txt = resp.text().await.unwrap_or_else(|e| {
                error!("Failed to read response text: {}", e);
                "".to_string()
            });
            debug!("Raw response body: {}", txt);

            if status.is_success() {
                // Spróbuj sparsować JSON
                match serde_json::from_str::<serde_json::Value>(&txt) {
                    Ok(json) => {
                        debug!("Parsed JSON: {:?}", json);

                        let token = json.get("token").and_then(|v| v.as_str()).map(|s| s.to_string());
                        let refresh_token = json.get("refresh_token").and_then(|v| v.as_str()).map(|s| s.to_string());

                        if let (Some(token), Some(refresh_token)) = (token.clone(), refresh_token.clone()) {
                            let tokens = AuthTokens {
                                token,
                                refresh_token,
                            };
                            *TOKENS.write().unwrap() = Some(tokens);
                            info!("Tokens saved to TOKENS storage");
                        } else {
                            warn!("Missing tokens in login response JSON: {:?}", json);
                        }

                        AuthResponse {
                            success: true,
                            message: "Login successful!".to_string(),
                            token:"".to_string(),
                            refresh_token:"".to_string(),
                        }
                    }
                    Err(e) => {
                        error!("Failed to parse JSON: {}", e);
                        AuthResponse {
                            success: false,
                            message: format!("Failed to parse server response: {}", e),
                            token: "".to_string(),
                            refresh_token: "".to_string(),
                        }
                    }
                }
            } else {
                error!("Login failed: status={} body={}", status, txt);
                AuthResponse {
                    success: false,
                    message: format!("Error: {} ({})", txt, status),
                    token: "".to_string(),
                    refresh_token: "".to_string(),
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
    info!("Register called with email: {}", email);

    let client = reqwest::Client::new();
    let api_url = std::env::var("API_URL").unwrap_or_else(|_| "http://139.59.138.164:3000".to_string());

    let payload = serde_json::json!({
        "email": email,
        "password": password,
    });

    let url = format!("{}/api/auth/register", api_url);

    debug!("Sending POST to {} with payload: {}", url, payload);

    let res = client.post(&url)
        .json(&payload)
        .send()
        .await;

    match res {
        Ok(resp) => {
            debug!("Server response status: {}", resp.status());

            let status = resp.status();
            let txt = resp.text().await.unwrap_or_else(|e| {
                error!("Failed to read response text: {}", e);
                "".to_string()
            });
            debug!("Raw response body: {}", txt);

            if status.is_success() {
                info!("Registration successful for email: {}", email);
                AuthResponse {
                    success: true,
                    message: "Registration successful!".to_string(),
                    token: "".to_string(),
                    refresh_token: "".to_string(),
                }
            } else {
                error!("Registration failed: status={} body={}", status, txt);
                AuthResponse {
                    success: false,
                    message: format!("Error: {} ({})", txt, status),
                    token: "".to_string(),
                    refresh_token: "".to_string(),
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