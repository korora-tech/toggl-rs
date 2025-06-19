use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_create_desktop_login_token() -> Result<()> {
    let response = json!({
        "token": "abc123def456",
        "confirmation_code": "12345"
    });

    with_mockito(
        Method::POST,
        "/desktop_login_tokens",
        200,
        Some(response),
        |client| {
            let token = client.desktop_login().create_token()?;
            assert_eq!("abc123def456", token.token);
            assert_eq!("12345", token.confirmation_code);
            Ok(())
        },
    )
}

#[test]
fn test_confirm_desktop_login() -> Result<()> {
    let response = json!({
        "user_id": 123456,
        "api_token": "user_api_token_here"
    });

    with_mockito(
        Method::POST,
        "/desktop_login",
        200,
        Some(response),
        |client| {
            let result = client.desktop_login().confirm("12345")?;
            assert_eq!(123456, result.user_id);
            assert_eq!("user_api_token_here", result.api_token);
            Ok(())
        },
    )
}
