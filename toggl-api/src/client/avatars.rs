use super::TogglClient;
use crate::models::api::avatar::Avatar;
use reqwest::Method;
use toggl_core::Result;

pub struct AvatarsClient {
    client: TogglClient,
}

impl AvatarsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get avatar URLs
    pub fn get(&self) -> Result<Avatar> {
        self.client.request(Method::GET, "avatars")
    }

    /// Use gravatar
    pub fn use_gravatar(&self) -> Result<Avatar> {
        self.client
            .request_empty(Method::POST, "avatars/use_gravatar")?;
        self.get()
    }
}
