use super::TogglClient;
use crate::error::Result;
use crate::model::api::keys::ApiKey;
use reqwest::Method;

pub struct KeysClient {
    client: TogglClient,
}

impl KeysClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get API keys
    pub fn list(&self) -> Result<Vec<ApiKey>> {
        self.client.request(Method::GET, "keys")
    }

    /// Create API key
    pub fn create(&self, description: Option<&str>) -> Result<ApiKey> {
        let body = serde_json::json!({
            "description": description
        });
        self.client.request_with_body(Method::POST, "keys", &body)
    }

    /// Delete API key
    pub fn delete(&self, key_id: &str) -> Result<()> {
        self.client
            .request_empty(Method::DELETE, &format!("keys/{}", key_id))
    }
}
