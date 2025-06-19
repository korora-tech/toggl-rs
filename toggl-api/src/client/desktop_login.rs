use super::TogglClient;
use reqwest::Method;
use serde::{Deserialize, Serialize};
use toggl_core::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopLoginToken {
    pub token: String,
    pub confirmation_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopLoginRequest {
    pub confirmation_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopLoginResponse {
    pub user_id: u64,
    pub api_token: String,
}

pub struct DesktopLoginClient {
    client: TogglClient,
}

impl DesktopLoginClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Create desktop login token
    pub fn create_token(&self) -> Result<DesktopLoginToken> {
        self.client.request(Method::POST, "desktop_login_tokens")
    }

    /// Confirm desktop login
    pub fn confirm(&self, confirmation_code: &str) -> Result<DesktopLoginResponse> {
        let body = DesktopLoginRequest {
            confirmation_code: confirmation_code.to_string(),
        };
        self.client
            .request_with_body(Method::POST, "desktop_login", &body)
    }
}
