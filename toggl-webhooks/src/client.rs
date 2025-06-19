//! Toggl Webhooks API client implementation

use toggl_core::Auth;

/// Toggl Webhooks API client
#[allow(dead_code)]
pub struct WebhooksClient {
    auth: Auth,
    base_url: String,
    client: reqwest::Client,
}

impl WebhooksClient {
    /// Create a new Webhooks API client
    pub fn new(auth: Auth) -> Self {
        Self {
            auth,
            base_url: "https://api.track.toggl.com/webhooks/api/v1".to_string(),
            client: reqwest::Client::new(),
        }
    }
}
