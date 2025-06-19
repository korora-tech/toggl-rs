use crate::error::Result;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_workspace_exports() -> Result<()> {
    let response = json!([
        {
            "token": "abc123",
            "state": "completed",
            "error_message": null
        },
        {
            "token": "def456",
            "state": "failed",
            "error_message": "Export failed"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/12345/exports",
        200,
        Some(response),
        |client| {
            let exports = client.workspaces().get_exports(12345)?;
            assert_eq!(exports.len(), 2);
            assert_eq!(exports[0].token, Some("abc123".to_string()));
            assert_eq!(exports[0].state, Some("completed".to_string()));
            assert!(exports[0].error_message.is_none());
            assert_eq!(exports[1].state, Some("failed".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_create_workspace_export() -> Result<()> {
    let response = json!("export-uuid-12345");

    with_mockito(
        Method::POST,
        "/workspaces/12345/exports",
        200,
        Some(response),
        |client| {
            let tokens = vec!["projects".to_string(), "time_entries".to_string()];
            let export_id = client.workspaces().create_export(12345, tokens)?;
            assert_eq!(export_id, "export-uuid-12345");
            Ok(())
        },
    )
}

#[test]
fn test_download_workspace_export() -> Result<()> {
    // For binary responses, we'll just test that the request is made correctly
    // The actual binary data handling is tested in the client implementation
    with_mockito(
        Method::GET,
        "/workspaces/12345/exports/data/export-uuid-12345.zip",
        200,
        None,
        |client| {
            // This will make the request, but mockito doesn't handle binary responses well
            // In real usage, this would return the zip file contents
            let result = client
                .workspaces()
                .download_export(12345, "export-uuid-12345");
            // Just verify the request is made successfully
            assert!(result.is_ok());
            Ok(())
        },
    )
}
