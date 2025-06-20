//! Tests for health endpoint

use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_get_health_ok() -> Result<()> {
    let response = json!({
        "status": "ok",
        "description": "All systems operational",
        "msg": "Webhooks API is healthy"
    });

    with_mockito(
        Method::GET,
        "/api/v9/health",
        200,
        Some(response),
        |client| {
            let health = client.get_health()?;
            assert_eq!(health.status, Some("ok".to_string()));
            assert_eq!(
                health.description,
                Some("All systems operational".to_string())
            );
            assert_eq!(health.msg, Some("Webhooks API is healthy".to_string()));
            assert!(health.error.is_none());
            Ok(())
        },
    )
}

#[test]
fn test_get_health_degraded() -> Result<()> {
    let response = json!({
        "status": "degraded",
        "description": "Partial system failure",
        "msg": "Some services are experiencing issues",
        "error": "Database connection pool exhausted"
    });

    with_mockito(
        Method::GET,
        "/api/v9/health",
        200,
        Some(response),
        |client| {
            let health = client.get_health()?;
            assert_eq!(health.status, Some("degraded".to_string()));
            assert_eq!(
                health.description,
                Some("Partial system failure".to_string())
            );
            assert_eq!(
                health.error,
                Some("Database connection pool exhausted".to_string())
            );
            Ok(())
        },
    )
}

#[test]
fn test_get_health_server_error() -> Result<()> {
    let error_response = json!({
        "code": 500,
        "message": "Internal server error"
    });

    with_mockito(
        Method::GET,
        "/api/v9/health",
        500,
        Some(error_response),
        |client| match client.get_health() {
            Err(e) => {
                assert!(e.to_string().contains("Internal server error"));
                Ok(())
            }
            Ok(_) => panic!("Expected error but got success"),
        },
    )
}
