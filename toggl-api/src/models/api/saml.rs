use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub sso_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSOConfirmation {
    pub confirmation_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedSsoProfile {
    pub domain: Option<String>,
    pub name: Option<String>,
    pub sso_profile_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkSsoProfile {
    pub domain: String,
}
