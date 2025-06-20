use super::TogglClient;
use crate::models::api::auth::SamlLoginResponse;
use crate::models::api::ids::WorkspaceId;
use reqwest::Method;
use toggl_core::Result;

pub struct AuthClient {
    client: TogglClient,
}

impl AuthClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Initiate SAML2 login
    pub fn saml2_login(&self, workspace_id: Option<WorkspaceId>) -> Result<SamlLoginResponse> {
        let path = match workspace_id {
            Some(id) => format!("auth/saml2/login/{}", id),
            None => "auth/saml2/login".to_string(),
        };
        self.client.request(Method::POST, &path)
    }
}
