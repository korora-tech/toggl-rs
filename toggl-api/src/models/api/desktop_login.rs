use serde::{Deserialize, Serialize};

use super::ids::UserId;

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
    pub user_id: UserId,
    pub api_token: String,
}
