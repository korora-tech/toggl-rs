use super::TogglClient;
use crate::models::api::invitation::Invitation;
use reqwest::Method;
use toggl_core::Result;

pub struct InvitationsClient {
    client: TogglClient,
}

impl InvitationsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get invitation details
    pub fn get(&self, invitation_code: &str) -> Result<Invitation> {
        self.client
            .request(Method::GET, &format!("invitations/{}", invitation_code))
    }
}
