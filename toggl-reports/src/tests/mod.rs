#[cfg(test)]
use crate::client::ReportsClient;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::test_utils::{
    with_mock_server, with_mock_server_bytes, with_mock_server_params, TEST_API_TOKEN,
};
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
    with_mock_server(
        |base_url| {
            ReportsClient::new_with_base_urls(TEST_API_TOKEN.to_string(), base_url, base_url)
        },
        method.as_str(),
        url,
        status,
        response,
        test,
    )
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
    if let Some(params) = params {
        with_mock_server_params(
            |base_url| {
                ReportsClient::new_with_base_urls(TEST_API_TOKEN.to_string(), base_url, base_url)
            },
            method.as_str(),
            path,
            params,
            status,
            response,
            test,
        )
    } else {
        with_mock_server(
            |base_url| {
                ReportsClient::new_with_base_urls(TEST_API_TOKEN.to_string(), base_url, base_url)
            },
            method.as_str(),
            path,
            status,
            response,
            test,
        )
    }
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
    with_mock_server_bytes(
        |base_url| {
            ReportsClient::new_with_base_urls(TEST_API_TOKEN.to_string(), base_url, base_url)
        },
        method.as_str(),
        url,
        status,
        response_bytes,
        test,
    )
}
