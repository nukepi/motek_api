#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: Option<String>,
    pub refresh_token: Option<String>,
}