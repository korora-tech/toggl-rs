//! Toggl Reports API client implementation

use toggl_core::Auth;

/// Toggl Reports API client
#[allow(dead_code)]
pub struct ReportsClient {
    auth: Auth,
    base_url: String,
    client: reqwest::Client,
}

impl ReportsClient {
    /// Create a new Reports API client
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            base_url: "https://api.track.toggl.com/reports/api/v3".to_string(),
            client: reqwest::Client::new(),
        }
    }
}
