use super::TogglClient;
use crate::models::api::ids::{SsoProfileId, WorkspaceId};
use crate::models::api::saml::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct SamlClient {
    client: TogglClient,
}

impl SamlClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get SAML2 Identity Provider URL
    pub fn saml2_login(&self, email: &str) -> Result<LoginResponse> {
        let mut params = BTreeMap::new();
        params.insert("email".to_string(), email.to_string());

        self.client
            .request_with_params(Method::GET, "auth/saml2/login", &params)
    }

    /// SAML2 Identity Provider Callback
    pub fn saml2_callback(
        &self,
        workspace_id: WorkspaceId,
        saml_response: &str,
        relay_state: Option<&str>,
    ) -> Result<()> {
        let mut body = BTreeMap::new();
        body.insert("SAMLResponse", saml_response);
        if let Some(relay_state) = relay_state {
            body.insert("RelayState", relay_state);
        }

        self.client.request_with_body_empty(
            Method::POST,
            &format!("auth/saml2/login/{}", workspace_id),
            &body,
        )
    }

    /// Confirm SSO enabling for user account
    pub fn enable_sso(&self, confirmation: &SSOConfirmation) -> Result<()> {
        self.client
            .request_with_body_empty(Method::POST, "me/enable_sso", confirmation)
    }

    /// Get linked SSO profiles for workspace
    pub fn get_linked_sso_profiles(
        &self,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<LinkedSsoProfile>> {
        self.client.request(
            Method::GET,
            &format!("workspaces/{}/linked_sso_profiles", workspace_id),
        )
    }

    /// Link SSO profile to workspace
    pub fn link_sso_profile(
        &self,
        workspace_id: WorkspaceId,
        sso_profile_id: SsoProfileId,
        link_profile: &LinkSsoProfile,
    ) -> Result<()> {
        self.client.request_with_body_empty(
            Method::PUT,
            &format!(
                "workspaces/{}/linked_sso_profiles/{}",
                workspace_id, sso_profile_id
            ),
            link_profile,
        )
    }

    /// Unlink SSO profile from workspace
    pub fn unlink_sso_profile(
        &self,
        workspace_id: WorkspaceId,
        sso_profile_id: SsoProfileId,
    ) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!(
                "workspaces/{}/linked_sso_profiles/{}",
                workspace_id, sso_profile_id
            ),
        )
    }
}
