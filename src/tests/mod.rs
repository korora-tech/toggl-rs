use crate::{client::TogglClient, error::Result};
use mockito::{Matcher, Server};
use reqwest::Method;
use std::collections::BTreeMap;

pub mod audit_logs;
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
pub mod me_favorites;
pub mod me_time_entries;
pub mod me_timesheets;
pub mod organization_groups;
pub mod organization_ownership_transfer;
pub mod organization_subscription;
pub mod organizations;
pub mod projects;
pub mod projects_advanced;
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
pub mod workspace_alerts;
pub mod workspace_clients_bulk;
pub mod workspace_dashboard;
pub mod workspace_expenses;
pub mod workspace_exports;
pub mod workspace_favorites;
pub mod workspace_groups;
pub mod workspace_preferences;
pub mod workspace_project_groups;
pub mod workspace_project_users;
pub mod workspace_rates;
pub mod workspace_scheduled_reports;
pub mod workspace_shared_reports;
pub mod workspace_subscription;
pub mod workspace_time_entry_constraints;
pub mod workspaces;
pub mod workspaces_goals;

const API_TOKEN: &str = "test_api_token";

pub fn with_mockito<F, T>(
    method: Method,
    url: &str,
    status: usize,
    response: Option<serde_json::Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(TogglClient) -> Result<T>,
{
    with_mockito_params(method, url, None, status, response, test)
}

pub fn with_mockito_params<F, T>(
    method: Method,
    path: &str,
    params: Option<BTreeMap<&str, &str>>,
    status: usize,
    response: Option<serde_json::Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(TogglClient) -> Result<T>,
{
    let mut server = Server::new();

    // Build the full URL with query parameters if provided
    let url = if let Some(params) = params {
        if params.is_empty() {
            path.to_string()
        } else {
            let query_string = params
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("&");
            format!("{}?{}", path, query_string)
        }
    } else {
        path.to_string()
    };

    let mock = server
        .mock(method.as_ref(), url.as_str())
        .match_header("authorization", Matcher::Any)
        .with_status(status);

    let mock = if let Some(response) = response {
        mock.with_body(response.to_string())
    } else {
        mock
    };

    let mock = mock.create();

    // Create a custom client that points to the mock server
    let mock_url = server.url();
    let toggl_client = TogglClient::new_with_base_url(API_TOKEN.to_string(), &mock_url)?;

    let result = test(toggl_client);

    mock.assert();

    result
}
