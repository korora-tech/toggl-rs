use super::TogglClient;
use crate::error::Result;
use reqwest::Method;

pub struct ICalClient {
    client: TogglClient,
}

impl ICalClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get iCal file for a workspace user
    /// Returns iCal file content with time entries from last 14 days
    pub fn get_ical_file(&self, token: &str) -> Result<String> {
        self.client
            .request(Method::GET, &format!("ical/workspace_user/{}", token))
    }

    /// Reset the iCal token for a workspace
    /// Returns the new token
    pub fn reset_token(&self, workspace_id: u64) -> Result<String> {
        self.client.request(
            Method::POST,
            &format!("workspaces/{}/ical/reset", workspace_id),
        )
    }

    /// Toggle the iCal token on/off for a workspace
    /// Returns the status message
    pub fn toggle_ical(&self, workspace_id: u64) -> Result<String> {
        self.client.request(
            Method::POST,
            &format!("workspaces/{}/ical/toggle", workspace_id),
        )
    }
}
