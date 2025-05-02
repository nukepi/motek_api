use reqwest::Client;
use serde::de::DeserializeOwned;
use crate::utils::token::TOKENS;

const DEFAULT_API_URL: &str = "http://127.0.0.1:3000";

fn api_url() -> String {
    std::env::var("API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

pub async fn authorized_post<T: DeserializeOwned>(
    endpoint: &str,
    payload: serde_json::Value,
) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    let token = TOKENS.read().ok().and_then(|g| g.as_ref().map(|t| t.token.clone()))?;

    let url = format!("{}{}", api_url, endpoint);
    let res = client
        .post(url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .ok()?;

    res.json::<T>().await.ok()
}

pub async fn authorized_get<T: DeserializeOwned>(endpoint: &str) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    let token = TOKENS.read().ok().and_then(|g| g.as_ref().map(|t| t.token.clone()))?;

    let url = format!("{}{}", api_url, endpoint);
    let res = client
        .get(url)
        .bearer_auth(token)
        .send()
        .await
        .ok()?;

    res.json::<T>().await.ok()
}

pub async fn authorized_put<T: DeserializeOwned>(
    endpoint: &str,
    payload: serde_json::Value,
) -> Option<T> {
    let client = Client::new();
    let api_url = api_url();
    let token = TOKENS.read().ok().and_then(|g| g.as_ref().map(|t| t.token.clone()))?;

    let url = format!("{}{}", api_url, endpoint);
    let res = client
        .put(url)
        .bearer_auth(token)
        .json(&payload)
        .send()
        .await
        .ok()?;

    res.json::<T>().await.ok()
}

pub async fn authorized_delete(endpoint: &str) -> Option<bool> {
    let client = Client::new();
    let api_url = api_url();
    let token = TOKENS.read().ok().and_then(|g| g.as_ref().map(|t| t.token.clone()))?;

    let url = format!("{}{}", api_url, endpoint);
    let res = client
        .delete(url)
        .bearer_auth(token)
        .send()
        .await
        .ok()?;

    Some(res.status().is_success())
}
