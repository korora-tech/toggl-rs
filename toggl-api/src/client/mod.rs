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

use reqwest::{
    blocking::{Client, Response},
    Method,
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;

use toggl_core::{Error, Result};

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
    api_token: String,
    base_url: String,
    client: Client,
}

impl TogglClient {
    pub fn new(api_token: String) -> Result<Self> {
        Self::new_with_base_url(api_token, BASE_URL)
    }

    pub fn new_with_base_url(api_token: String, base_url: &str) -> Result<Self> {
        let client = Client::builder()
            .build()
            .map_err(|e| Error::NetworkError(format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            api_token,
            base_url: base_url.to_string(),
            client,
        })
    }

    fn auth_header(&self) -> String {
        use base64::Engine;
        let credentials = format!("{}:api_token", self.api_token);
        let encoded = base64::engine::general_purpose::STANDARD.encode(credentials);
        format!("Basic {}", encoded)
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
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        let response = self.send_request(method, &url, params, None::<&()>)?;
        self.handle_response(response)
    }

    pub(crate) fn request_with_body<T: DeserializeOwned, B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<T> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        let response = self.send_request(method, &url, &BTreeMap::new(), Some(body))?;
        self.handle_response(response)
    }

    pub(crate) fn request_empty(&self, method: Method, path: &str) -> Result<()> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        let response = self.send_request(method, &url, &BTreeMap::new(), None::<&()>)?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    pub(crate) fn request_with_body_empty<B: Serialize>(
        &self,
        method: Method,
        path: &str,
        body: &B,
    ) -> Result<()> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        let response = self.send_request(method, &url, &BTreeMap::new(), Some(body))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    #[allow(unused_mut)]
    pub(crate) fn request_binary(&self, method: Method, path: &str) -> Result<Vec<u8>> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        let mut response = self.send_request(method, &url, &BTreeMap::new(), None::<&()>)?;
        let status = response.status();

        if status.is_success() {
            response.bytes().map(|b| b.to_vec()).map_err(|e| {
                Error::ResponseParseError(format!("Failed to read response bytes: {}", e))
            })
        } else {
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    pub(crate) fn request_multipart(
        &self,
        method: Method,
        path: &str,
        file_data: &[u8],
        file_name: &str,
    ) -> Result<()> {
        self.request_multipart_with_mime(method, path, file_data, file_name, "text/csv")
    }

    pub(crate) fn request_multipart_with_mime(
        &self,
        method: Method,
        path: &str,
        file_data: &[u8],
        file_name: &str,
        mime_type: &str,
    ) -> Result<()> {
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        #[cfg(test)]
        println!("Sending multipart request to: {} {}", method, url);

        // Use reqwest's multipart support
        let part = reqwest::blocking::multipart::Part::bytes(file_data.to_vec())
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .map_err(|e| Error::RequestBuildError(format!("Failed to create multipart: {}", e)))?;

        let form = reqwest::blocking::multipart::Form::new().part("file", part);

        let response = self
            .client
            .request(method, &url)
            .header("Authorization", self.auth_header())
            .multipart(form)
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            Ok(())
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
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
        let url = if path.starts_with('/') {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}/{}", self.base_url, path)
        };

        #[cfg(test)]
        println!("Sending multipart JSON+file request to: {} {}", method, url);

        // Serialize JSON data
        let json_string = serde_json::to_string(json_data)
            .map_err(|e| Error::SerializationError(format!("Failed to serialize JSON: {}", e)))?;

        // Create multipart form with both JSON and file
        let json_part = reqwest::blocking::multipart::Part::text(json_string)
            .mime_str("application/json")
            .map_err(|e| Error::RequestBuildError(format!("Failed to create JSON part: {}", e)))?;

        let file_part = reqwest::blocking::multipart::Part::bytes(file_data.to_vec())
            .file_name(file_name.to_string())
            .mime_str(mime_type)
            .map_err(|e| Error::RequestBuildError(format!("Failed to create file part: {}", e)))?;

        let form = reqwest::blocking::multipart::Form::new()
            .part("data", json_part)
            .part(file_field_name.to_string(), file_part);

        let response = self
            .client
            .request(method, &url)
            .header("Authorization", self.auth_header())
            .multipart(form)
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))?;

        if response.status().is_success() {
            response.json::<T>().map_err(|e| {
                Error::ResponseParseError(format!("Failed to deserialize response: {}", e))
            })
        } else {
            let status = response.status();
            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());
            Err(Error::ApiError {
                status: status.as_u16(),
                message: error_text,
                details: None,
            })
        }
    }

    fn send_request<B: Serialize>(
        &self,
        method: Method,
        url: &str,
        params: &BTreeMap<String, String>,
        body: Option<&B>,
    ) -> Result<Response> {
        #[cfg(test)]
        println!("Sending request to: {} {}", method, url);

        let mut request = self
            .client
            .request(method, url)
            .header("Authorization", self.auth_header())
            .header("Content-Type", "application/json");

        // Add query parameters
        for (key, value) in params {
            request = request.query(&[(key, value)]);
        }

        // Add body if provided
        if let Some(body) = body {
            request = request.json(body);
        }

        request
            .send()
            .map_err(|e| Error::NetworkError(format!("Failed to send request: {}", e)))
    }

    #[allow(unused_mut)]
    fn handle_response<T: DeserializeOwned>(&self, mut response: Response) -> Result<T> {
        let status = response.status();

        if status.is_success() {
            response.json::<T>().map_err(|e| {
                Error::ResponseParseError(format!("Failed to deserialize JSON: {}", e))
            })
        } else {
            let status_code = status.as_u16();

            // Extract retry-after header before consuming response
            let retry_after = if status_code == 429 {
                response
                    .headers()
                    .get("retry-after")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| v.parse::<u64>().ok())
                    .unwrap_or(60) // Default to 60 seconds
            } else {
                0
            };

            let error_text = response
                .text()
                .unwrap_or_else(|_| "Unknown error".to_string());

            // Try to parse as JSON to get structured error details
            let details = serde_json::from_str::<serde_json::Value>(&error_text).ok();
            let message = if let Some(ref json) = details {
                // Try to extract message from common error response formats
                json.get("message")
                    .or_else(|| json.get("error"))
                    .and_then(|v| v.as_str())
                    .unwrap_or(&error_text)
                    .to_string()
            } else {
                error_text
            };

            // Map specific status codes to more specific error types
            match status_code {
                401 => Err(Error::AuthError(message)),
                403 => Err(Error::PermissionError(message)),
                404 => {
                    // Try to extract resource info from the path
                    Err(Error::ApiError {
                        status: status_code,
                        message,
                        details,
                    })
                }
                429 => Err(Error::RateLimitError { retry_after }),
                422 => Err(Error::ValidationError(message)),
                _ => Err(Error::ApiError {
                    status: status_code,
                    message,
                    details,
                }),
            }
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
