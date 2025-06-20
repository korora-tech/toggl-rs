use crate::error::Result;
use mockito::{Matcher, Mock, Server, ServerGuard};
use serde_json::Value;

pub fn with_mock_server<C, F, T>(
    create_client: impl FnOnce(&str) -> Result<C>,
    method: &str,
    path: &str,
    status: usize,
    response: Option<Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(C) -> Result<T>,
{
    let mut server = Server::new();
    let mock = setup_mock(&mut server, method, path, status, response);

    let client = create_client(&server.url())?;
    let result = test(client);

    mock.assert();
    result
}

pub fn with_mock_server_params<'a, C, F, T>(
    create_client: impl FnOnce(&str) -> Result<C>,
    method: &str,
    path: &str,
    params: impl IntoIterator<Item = (&'a str, &'a str)>,
    status: usize,
    response: Option<Value>,
    test: F,
) -> Result<T>
where
    F: FnOnce(C) -> Result<T>,
{
    let mut server = Server::new();
    let mock = setup_mock_with_params(&mut server, method, path, params, status, response);

    let client = create_client(&server.url())?;
    let result = test(client);

    mock.assert();
    result
}

pub fn with_mock_server_bytes<C, F, T>(
    create_client: impl FnOnce(&str) -> Result<C>,
    method: &str,
    path: &str,
    status: usize,
    response_bytes: Vec<u8>,
    test: F,
) -> Result<T>
where
    F: FnOnce(C) -> Result<T>,
{
    let mut server = Server::new();
    let mock = server
        .mock(method, path)
        .match_header("authorization", Matcher::Regex(r"^Basic\s.+".to_string()))
        .with_status(status)
        .with_body(response_bytes)
        .create();

    let client = create_client(&server.url())?;
    let result = test(client);

    mock.assert();
    result
}

fn setup_mock(
    server: &mut ServerGuard,
    method: &str,
    path: &str,
    status: usize,
    response: Option<Value>,
) -> Mock {
    let mock = server
        .mock(method, path)
        .match_header("authorization", Matcher::Regex(r"^Basic\s.+".to_string()))
        .with_status(status);

    if let Some(json) = response {
        mock.with_header("content-type", "application/json")
            .with_body(json.to_string())
            .create()
    } else {
        mock.create()
    }
}

fn setup_mock_with_params<'a>(
    server: &mut ServerGuard,
    method: &str,
    path: &str,
    params: impl IntoIterator<Item = (&'a str, &'a str)>,
    status: usize,
    response: Option<Value>,
) -> Mock {
    let mut mock = server
        .mock(method, path)
        .match_header("authorization", Matcher::Regex(r"^Basic\s.+".to_string()));

    for (key, value) in params {
        mock = mock.match_query(Matcher::UrlEncoded(key.to_string(), value.to_string()));
    }

    let mock = mock.with_status(status);

    if let Some(json) = response {
        mock.with_header("content-type", "application/json")
            .with_body(json.to_string())
            .create()
    } else {
        mock.create()
    }
}
