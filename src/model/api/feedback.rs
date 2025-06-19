use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackRequest {
    pub subject: String,
    pub details: String,
    pub toggl_version: Option<String>,
    pub os: Option<String>,
    pub os_version: Option<String>,
    pub device: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebFeedbackRequest {
    pub message: String,
    pub browser: Option<String>,
    pub browser_version: Option<String>,
    pub os: Option<String>,
    pub url: Option<String>,
}
