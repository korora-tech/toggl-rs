use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub api_token: String,
    pub created_at: String,
    pub expires_at: Option<String>,
    pub description: Option<String>,
}
