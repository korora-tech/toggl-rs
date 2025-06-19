use crate::error::Result;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::with_mockito;

#[test]
fn test_get_status() -> Result<()> {
    let response = json!({
        "status": "ok",
        "version": "9.1.0",
        "timestamp": "2023-01-01T12:00:00Z"
    });

    with_mockito(Method::GET, "/status", 200, Some(response), |client| {
        let status = client.status().get()?;
        assert_eq!("ok", status.status);
        assert_eq!("9.1.0", status.version);
        assert_eq!("2023-01-01T12:00:00Z", status.timestamp);
        Ok(())
    })
}
