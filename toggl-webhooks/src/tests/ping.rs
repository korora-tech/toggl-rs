//! Tests for ping endpoint

use reqwest::Method;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_ping_subscription() -> Result<()> {
    let subscription_id = "sub_123";

    with_mockito(Method::POST, "/api/v9/ping/sub_123", 204, None, |client| {
        client.ping_subscription(subscription_id)?;
        Ok(())
    })
}

#[test]
fn test_ping_subscription_not_found() -> Result<()> {
    let subscription_id = "sub_not_found";
    let error_response = serde_json::json!({
        "code": 404,
        "message": "Subscription not found"
    });

    with_mockito(
        Method::POST,
        "/api/v9/ping/sub_not_found",
        404,
        Some(error_response),
        |client| match client.ping_subscription(subscription_id) {
            Err(e) => {
                assert!(e.to_string().contains("Subscription not found"));
                Ok(())
            }
            Ok(_) => panic!("Expected error but got success"),
        },
    )
}

#[test]
fn test_ping_subscription_forbidden() -> Result<()> {
    let subscription_id = "sub_123";
    let error_response = serde_json::json!({
        "code": 403,
        "message": "User does not have permission to ping subscription"
    });

    with_mockito(
        Method::POST,
        "/api/v9/ping/sub_123",
        403,
        Some(error_response),
        |client| match client.ping_subscription(subscription_id) {
            Err(e) => {
                assert!(e.to_string().contains("permission"));
                Ok(())
            }
            Ok(_) => panic!("Expected error but got success"),
        },
    )
}
