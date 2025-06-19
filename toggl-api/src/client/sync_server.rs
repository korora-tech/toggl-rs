use super::TogglClient;
use crate::models::api::sync_server::Goal;
use reqwest::Method;
use toggl_core::Result;

pub struct SyncServerClient {
    client: TogglClient,
}

impl SyncServerClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get personal goals
    pub fn get_goals(&self) -> Result<Vec<Goal>> {
        self.client.request(Method::GET, "sync-server/me/goals")
    }
}
