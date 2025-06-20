#[cfg(test)]
mod tests {
    use super::super::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;

    #[test]
    fn test_get_shared_report() -> Result<()> {
        let response = json!({
            "token": "abc123xyz",
            "name": "Monthly Summary Report",
            "type_": "summary",
            "data": {
                "workspace_id": 123,
                "start_date": "2024-01-01",
                "end_date": "2024-01-31"
            }
        });

        with_mockito(
            Method::GET,
            "/shared/abc123xyz",
            200,
            Some(response),
            |client| {
                let report = client.shared().get("abc123xyz")?;
                assert_eq!(report.token, "abc123xyz");
                assert_eq!(report.name, "Monthly Summary Report");
                assert_eq!(report.type_, "summary");
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_shared_csv() -> Result<()> {
        let csv_data = b"Project,Hours,Amount\nProject A,40,1000.00".to_vec();

        with_mockito_bytes(
            Method::GET,
            "/shared/abc123xyz/csv",
            200,
            csv_data.clone(),
            |client| {
                let data = client.shared().export_csv("abc123xyz")?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_shared_pdf() -> Result<()> {
        let pdf_data = b"%PDF-1.4".to_vec();

        with_mockito_bytes(
            Method::GET,
            "/shared/abc123xyz/pdf",
            200,
            pdf_data.clone(),
            |client| {
                let data = client.shared().export_pdf("abc123xyz")?;
                assert!(data.starts_with(b"%PDF"));
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_shared_xlsx() -> Result<()> {
        let xlsx_data = vec![0x50, 0x4B, 0x03, 0x04]; // XLSX magic bytes

        with_mockito_bytes(
            Method::GET,
            "/shared/abc123xyz/xlsx",
            200,
            xlsx_data.clone(),
            |client| {
                let data = client.shared().export_xlsx("abc123xyz")?;
                assert_eq!(&data[0..4], &[0x50, 0x4B, 0x03, 0x04]);
                Ok(())
            },
        )
    }
}
