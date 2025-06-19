use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_list_api_keys() -> Result<()> {
    let response = json!([
        {
            "api_token": "key_123456",
            "created_at": "2023-01-01T00:00:00Z",
            "expires_at": "2024-01-01T00:00:00Z",
            "description": "Production API Key"
        },
        {
            "api_token": "key_789012",
            "created_at": "2023-06-01T00:00:00Z",
            "expires_at": null,
            "description": "Development API Key"
        }
    ]);

    with_mockito(Method::GET, "/keys", 200, Some(response), |client| {
        let keys = client.keys().list()?;
        assert_eq!(2, keys.len());
        assert_eq!("key_123456", keys[0].api_token);
        assert_eq!(Some("Production API Key".to_string()), keys[0].description);
        assert!(keys[1].expires_at.is_none());
        Ok(())
    })
}

#[test]
fn test_create_api_key() -> Result<()> {
    let response = json!({
        "api_token": "new_key_345678",
        "created_at": "2023-12-01T00:00:00Z",
        "expires_at": null,
        "description": "CI/CD Key"
    });

    with_mockito(Method::POST, "/keys", 200, Some(response), |client| {
        let key = client.keys().create(Some("CI/CD Key"))?;
        assert_eq!("new_key_345678", key.api_token);
        assert_eq!(Some("CI/CD Key".to_string()), key.description);
        Ok(())
    })
}

#[test]
fn test_delete_api_key() -> Result<()> {
    with_mockito(Method::DELETE, "/keys/key_to_delete", 200, None, |client| {
        client.keys().delete("key_to_delete")?;
        Ok(())
    })
}
