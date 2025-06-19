use super::TogglClient;
use crate::{error::Result, model::api::*};
use reqwest::Method;

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
