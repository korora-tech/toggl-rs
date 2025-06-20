//! Test module for webhook API

use reqwest::Method;
use toggl_core::test_utils::{with_mock_server, with_mock_server_params, TEST_API_TOKEN};
use toggl_core::Result;

use crate::WebhooksClient;

/// Create a mock server and execute a test with the client
pub fn with_mockito<F, T>(
    method: Method,
    path: &str,
    status: usize,
    response: Option<serde_json::Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(&WebhooksClient) -> Result<T>,
{
    with_mock_server(
        |base_url| WebhooksClient::new_with_base_url(TEST_API_TOKEN.to_string(), base_url),
        method.as_str(),
        path,
        status,
        response,
        |client| test(&client),
    )
}

/// Create a mock server with query params and execute a test with the client
pub fn with_mockito_params<'a, F, T>(
    method: Method,
    path: &str,
    params: Vec<(&'a str, &'a str)>,
    status: usize,
    response: Option<serde_json::Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(&WebhooksClient) -> Result<T>,
{
    with_mock_server_params(
        |base_url| WebhooksClient::new_with_base_url(TEST_API_TOKEN.to_string(), base_url),
        method.as_str(),
        path,
        params,
        status,
        response,
        |client| test(&client),
    )
}

mod event_filters;
mod events;
mod health;
mod ping;
mod subscriptions;
mod validation;
