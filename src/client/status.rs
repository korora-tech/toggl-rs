use super::TogglClient;
use crate::error::Result;
use crate::model::api::status::ApiStatus;
use reqwest::Method;

pub struct StatusClient {
    client: TogglClient,
}

impl StatusClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get API status
    pub fn get(&self) -> Result<ApiStatus> {
        self.client.request(Method::GET, "status")
    }
}
