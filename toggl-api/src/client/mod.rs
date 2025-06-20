//! Toggl API client implementation
//!
//! This module provides the main `TogglClient` struct and all API endpoint
//! implementations organized by domain.
//!
//! # Client Structure
//!
//! Each API domain has its own client module that wraps the main `TogglClient`:
//! - `auth` - Authentication endpoints
//! - `me` - Current user endpoints
//! - `workspaces` - Workspace management
//! - `projects` - Project management
//! - `time_entries` - Time tracking
//! - And many more...
//!
//! # Usage
//!
//! ```rust,no_run
//! use toggl_api::TogglClient;
//!
//! let client = TogglClient::new("your-api-token".to_string()).unwrap();
//!
//! // Access different API domains
//! let me = client.me().get().unwrap();
//! let workspace = client.workspaces().get(12345).unwrap();
//! ```

use reqwest::Method;
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

use toggl_core::{BaseClient, Result};

pub mod audit;
pub mod auth;
pub mod avatars;
pub mod calendar;
pub mod countries;
pub mod currencies;
pub mod desktop_login;
pub mod exports;
pub mod favorites;
pub mod feedback;
pub mod goals;
pub mod ical;
pub mod invitations;
pub mod invoices;
pub mod keys;
pub mod me;
pub mod organizations;
pub mod projects;
pub mod reports;
pub mod saml;
pub mod scheduled_reports;
pub mod shared_reports;
pub mod smail;
pub mod status;
pub mod subscriptions;
pub mod sync_server;
pub mod time_entries;
pub mod time_entry_invitations;
pub mod timeline;
pub mod timesheets;
pub mod timezones;
pub mod workspace_reports;
pub mod workspaces;

const BASE_URL: &str = "https://api.track.toggl.com/api/v9";

#[derive(Clone)]
pub struct TogglClient {
    base_url: String,
    base_client: BaseClient,
}

impl TogglClient {
    pub fn new(api_token: String) -> Result<Self> {
        Self::new_with_base_url(api_token, BASE_URL)
    }

    pub fn new_with_base_url(api_token: String, base_url: &str) -> Result<Self> {
        let base_client = BaseClient::new(api_token)?;
        Ok(Self {
            base_url: base_url.to_string(),
            base_client,
        })
    }

    pub(crate) fn request<T: DeserializeOwned>(&self, method: Method, path: &str) -> Result<T> {
        self.request_with_params(method, path, &BTreeMap::new())
    }

    pub(crate) fn request_with_params<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<T> {
        let url = self.build_url(path);
        self.base_client.request_with_params(method, &url, params)
    }

    pub(crate) fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = self.build_url(path);
        self.base_client.request_with_body(method, &url, body)
    }

    pub(crate) fn request_empty(&self, method: Method, path: &str) -> Result<()> {
        let url = self.build_url(path);
        self.base_client.request_empty(method, &url)
    }

    pub(crate) fn request_with_body_empty<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<()> {
        let url = self.build_url(path);
        self.base_client.request_with_body_empty(method, &url, body)
    }

    pub(crate) fn request_binary(&self, method: Method, path: &str) -> Result<Vec<u8>> {
        let url = self.build_url(path);
        self.base_client.request_binary(method, &url)
    }

    pub(crate) fn request_multipart(
        &self,
        method: Method,
        path: &str,
        file_data: &[u8],
        file_name: &str,
    ) -> Result<()> {
        let url = self.build_url(path);
        self.base_client
            .request_multipart(method, &url, file_data, file_name)
    }

    pub(crate) fn request_multipart_with_mime(
        &self,
        method: Method,
        path: &str,
        file_data: &[u8],
        file_name: &str,
        mime_type: &str,
    ) -> Result<()> {
        let url = self.build_url(path);
        self.base_client
            .request_multipart_with_mime(method, &url, file_data, file_name, mime_type)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn request_multipart_json_file<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        json_data: &B,
        file_data: &[u8],
        file_name: &str,
        file_field_name: &str,
        mime_type: &str,
    ) -> Result<T> {
        let url = self.build_url(path);
        self.base_client.request_multipart_json_file(
            method,
            &url,
            json_data,
            file_data,
            file_name,
            file_field_name,
            mime_type,
        )
    }

    fn build_url(&self, path: &str) -> String {
        if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        }
    }

    // Client accessors
    pub fn audit(&self) -> audit::AuditClient {
        audit::AuditClient::new(self.clone())
    }

    pub fn auth(&self) -> auth::AuthClient {
        auth::AuthClient::new(self.clone())
    }

    pub fn calendar(&self) -> calendar::CalendarClient {
        calendar::CalendarClient::new(self.clone())
    }

    pub fn exports(&self) -> exports::ExportsClient {
        exports::ExportsClient::new(self.clone())
    }

    pub fn favorites(&self) -> favorites::FavoritesClient {
        favorites::FavoritesClient::new(self.clone())
    }

    pub fn goals(&self) -> goals::GoalsClient {
        goals::GoalsClient::new(self.clone())
    }

    pub fn me(&self) -> me::MeClient {
        me::MeClient::new(self.clone())
    }

    pub fn organizations(&self) -> organizations::OrganizationsClient {
        organizations::OrganizationsClient::new(self.clone())
    }

    pub fn workspaces(&self) -> workspaces::WorkspacesClient {
        workspaces::WorkspacesClient::new(self.clone())
    }

    pub fn projects(&self) -> projects::ProjectsClient {
        projects::ProjectsClient::new(self.clone())
    }

    pub fn time_entries(&self) -> time_entries::TimeEntriesClient {
        time_entries::TimeEntriesClient::new(self.clone())
    }

    pub fn reports(&self) -> reports::ReportsClient {
        reports::ReportsClient::new(self.clone())
    }

    pub fn countries(&self) -> countries::CountriesClient {
        countries::CountriesClient::new(self.clone())
    }

    pub fn currencies(&self) -> currencies::CurrenciesClient {
        currencies::CurrenciesClient::new(self.clone())
    }

    pub fn timezones(&self) -> timezones::TimezonesClient {
        timezones::TimezonesClient::new(self.clone())
    }

    pub fn invitations(&self) -> invitations::InvitationsClient {
        invitations::InvitationsClient::new(self.clone())
    }

    pub fn avatars(&self) -> avatars::AvatarsClient {
        avatars::AvatarsClient::new(self.clone())
    }

    pub fn feedback(&self) -> feedback::FeedbackClient {
        feedback::FeedbackClient::new(self.clone())
    }

    pub fn status(&self) -> status::StatusClient {
        status::StatusClient::new(self.clone())
    }

    pub fn desktop_login(&self) -> desktop_login::DesktopLoginClient {
        desktop_login::DesktopLoginClient::new(self.clone())
    }

    pub fn keys(&self) -> keys::KeysClient {
        keys::KeysClient::new(self.clone())
    }

    pub fn smail(&self) -> smail::SmailClient {
        smail::SmailClient::new(self.clone())
    }

    pub fn subscriptions(&self) -> subscriptions::SubscriptionsClient {
        subscriptions::SubscriptionsClient::new(self.clone())
    }

    pub fn sync_server(&self) -> sync_server::SyncServerClient {
        sync_server::SyncServerClient::new(self.clone())
    }

    pub fn timeline(&self) -> timeline::TimelineClient {
        timeline::TimelineClient::new(self.clone())
    }

    pub fn timesheets(&self) -> timesheets::TimesheetsClient {
        timesheets::TimesheetsClient::new(self.clone())
    }

    pub fn invoices(&self) -> invoices::InvoicesClient {
        invoices::InvoicesClient::new(self.clone())
    }

    pub fn saml(&self) -> saml::SamlClient {
        saml::SamlClient::new(self.clone())
    }

    pub fn workspace_reports(&self) -> workspace_reports::WorkspaceReportsClient {
        workspace_reports::WorkspaceReportsClient::new(self.clone())
    }

    pub fn ical(&self) -> ical::ICalClient {
        ical::ICalClient::new(self.clone())
    }

    pub fn shared_reports(&self) -> shared_reports::SharedReportsClient {
        shared_reports::SharedReportsClient::new(self.clone())
    }

    pub fn scheduled_reports(&self) -> scheduled_reports::ScheduledReportsClient {
        scheduled_reports::ScheduledReportsClient::new(self.clone())
    }

    pub fn time_entry_invitations(&self) -> time_entry_invitations::TimeEntryInvitationsClient {
        time_entry_invitations::TimeEntryInvitationsClient::new(self.clone())
    }

    // Convenience methods for simpler syntax
    pub(crate) fn get<T: DeserializeOwned>(
        &self,
        path: &str,
        params: &BTreeMap<String, String>,
    ) -> Result<T> {
        self.request_with_params(Method::GET, path, params)
    }

    pub(crate) fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request_with_body(Method::POST, path, body)
    }

    pub(crate) fn put<T: DeserializeOwned, B: Serialize>(&self, path: &str, body: &B) -> Result<T> {
        self.request_with_body(Method::PUT, path, body)
    }

    #[allow(dead_code)]
    pub(crate) fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T> {
        self.request_with_body(Method::PATCH, path, body)
    }

    #[allow(dead_code)]
    pub(crate) fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request(Method::DELETE, path)
    }

    pub(crate) fn delete_empty(&self, path: &str) -> Result<()> {
        self.request_empty(Method::DELETE, path)
    }
}
