#[cfg(test)]
use crate::client::ReportsClient;
use mockito::{Matcher, Server};
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub mod comparative;
pub mod data_trends;
pub mod detailed;
pub mod filters;
pub mod insights;
pub mod profitability;
pub mod projects;
pub mod search;
pub mod shared;
pub mod summary;
pub mod weekly;

const API_TOKEN: &str = "test_api_token";

pub fn with_mockito<F, T>(
    method: Method,
    url: &str,
    status: usize,
    response: Option<serde_json::Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(ReportsClient) -> Result<T>,
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
    F: FnOnce(ReportsClient) -> Result<T>,
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
        .mock(method.as_str(), url.as_str())
        .match_header("authorization", Matcher::Any)
        .with_status(status);

    let mock = if let Some(response) = response {
        mock.with_header("content-type", "application/json")
            .with_body(response.to_string())
    } else {
        mock
    };

    let _m = mock.create();

    let client =
        ReportsClient::new_with_base_urls(API_TOKEN.to_string(), &server.url(), &server.url())?;

    test(client)
}

pub fn with_mockito_bytes<F, T>(
    method: Method,
    url: &str,
    status: usize,
    response_bytes: Vec<u8>,
    test: F,
) -> Result<T>
where
    F: FnOnce(ReportsClient) -> Result<T>,
{
    let mut server = Server::new();

    let _m = server
        .mock(method.as_str(), url)
        .match_header("authorization", Matcher::Any)
        .with_status(status)
        .with_body(response_bytes)
        .create();

    let client =
        ReportsClient::new_with_base_urls(API_TOKEN.to_string(), &server.url(), &server.url())?;

    test(client)
}
