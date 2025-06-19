use super::TogglClient;
use crate::models::api::status::ApiStatus;
use reqwest::Method;
use toggl_core::Result;

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
