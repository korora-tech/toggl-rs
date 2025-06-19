#[cfg(test)]
mod tests {
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_request_export() -> Result<()> {
        let response = json!({
            "status": "pending",
            "export_id": "abc123"
        });

        with_mockito(Method::POST, "/me/export", 200, Some(response), |client| {
            let result = client.exports().request_export("invoices")?;

            assert_eq!(result.status, "pending");
            assert_eq!(result.export_id, Some("abc123".to_string()));
            Ok(())
        })
    }

    #[test]
    fn test_get_status() -> Result<()> {
        let response = json!({
            "status": "completed",
            "export_id": "abc123"
        });

        with_mockito(Method::GET, "/me/export", 200, Some(response), |client| {
            let result = client.exports().get_status()?;

            assert_eq!(result.status, "completed");
            assert_eq!(result.export_id, Some("abc123".to_string()));
            Ok(())
        })
    }

    #[test]
    fn test_download() -> Result<()> {
        // We need to use mockito's mock builder directly for binary responses
        let mut server = mockito::Server::new();
        let mock_url = server.url();

        // Mock binary data for a zip file
        let mock_zip_data = vec![0x50, 0x4B, 0x03, 0x04, 0x14, 0x00]; // ZIP file header

        let mock = server
            .mock("GET", "/me/export/data/abc123.zip")
            .match_header("authorization", mockito::Matcher::Any)
            .with_status(200)
            .with_body(&mock_zip_data)
            .create();

        // Create a client pointing to the mock server
        let client =
            crate::client::TogglClient::new_with_base_url("test_api_token".to_string(), &mock_url)?;

        let result = client.exports().download("abc123")?;

        assert_eq!(result, mock_zip_data);
        mock.assert();
        Ok(())
    }
}
