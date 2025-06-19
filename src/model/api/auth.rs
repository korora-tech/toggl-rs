use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SamlLoginRequest {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SamlLoginResponse {
    pub sso_url: String,
}
