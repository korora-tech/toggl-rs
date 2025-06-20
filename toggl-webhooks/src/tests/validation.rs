//! Tests for subscription validation endpoint

use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use crate::models::ValidationRequest;

use super::with_mockito;

#[test]
fn test_validate_subscription() -> Result<()> {
    let validation_request = ValidationRequest {
        event_type: Some("subscription.validation".to_string()),
        validation_code: Some("ABC123XYZ".to_string()),
    };

    let response = json!({
        "validation_code": "ABC123XYZ"
    });

    // Note: This endpoint doesn't use the /api/v9 prefix
    with_mockito(
        Method::POST,
        "/subscription_validation_events",
        200,
        Some(response),
        |client| {
            let validation_response = client.validate_subscription(&validation_request)?;
            assert_eq!(
                validation_response.validation_code,
                Some("ABC123XYZ".to_string())
            );
            Ok(())
        },
    )
}

#[test]
fn test_validate_subscription_empty() -> Result<()> {
    let validation_request = ValidationRequest {
        event_type: None,
        validation_code: None,
    };

    let response = json!({});

    with_mockito(
        Method::POST,
        "/subscription_validation_events",
        200,
        Some(response),
        |client| {
            let validation_response = client.validate_subscription(&validation_request)?;
            assert!(validation_response.validation_code.is_none());
            Ok(())
        },
    )
}

#[test]
fn test_validate_subscription_with_different_code() -> Result<()> {
    let validation_request = ValidationRequest {
        event_type: Some("subscription.validation".to_string()),
        validation_code: Some("REQUEST_CODE".to_string()),
    };

    let response = json!({
        "validation_code": "RESPONSE_CODE"
    });

    with_mockito(
        Method::POST,
        "/subscription_validation_events",
        200,
        Some(response),
        |client| {
            let validation_response = client.validate_subscription(&validation_request)?;
            // The response code might be different from the request code
            // This tests that we handle that case correctly
            assert_eq!(
                validation_response.validation_code,
                Some("RESPONSE_CODE".to_string())
            );
            Ok(())
        },
    )
}
