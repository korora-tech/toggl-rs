use crate::error::Result;
use crate::model::api::*;
use serde_json::json;

use super::with_mockito;
use reqwest::Method;

#[test]
fn test_list_scheduled_reports() -> Result<()> {
    let response = json!([
        {
            "bookmark_id": 123,
            "created_at": "2024-01-01T00:00:00Z",
            "creator_id": 456,
            "frequency": 7,
            "group_ids": [789],
            "report_id": 999,
            "user_ids": [456, 457],
            "workspace_id": 111
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/111/scheduled_reports",
        200,
        Some(response),
        |client| {
            let reports = client.scheduled_reports().list_scheduled_reports(111)?;
            assert_eq!(reports.len(), 1);
            assert_eq!(reports[0].bookmark_id, Some(123));
            assert_eq!(reports[0].frequency, Some(7));
            assert_eq!(reports[0].workspace_id, Some(111));
            Ok(())
        },
    )
}

#[test]
fn test_create_scheduled_report() -> Result<()> {
    let request = CreateScheduledReportPayload {
        bookmark_id: 123,
        frequency: 7,
        group_ids: Some(vec![789]),
        user_ids: Some(vec![456, 457]),
    };

    let response = json!({
        "bookmark_id": 123,
        "created_at": "2024-01-01T00:00:00Z",
        "creator_id": 456,
        "frequency": 7,
        "group_ids": [789],
        "report_id": 999,
        "user_ids": [456, 457],
        "workspace_id": 111
    });

    with_mockito(
        Method::POST,
        "/workspaces/111/scheduled_reports",
        200,
        Some(response),
        |client| {
            let report = client
                .scheduled_reports()
                .create_scheduled_report(111, request)?;
            assert_eq!(report.bookmark_id, Some(123));
            assert_eq!(report.frequency, Some(7));
            assert_eq!(report.report_id, Some(999));
            Ok(())
        },
    )
}

#[test]
fn test_delete_scheduled_report() -> Result<()> {
    let response = json!("OK");

    with_mockito(
        Method::DELETE,
        "/workspaces/111/scheduled_reports/999",
        200,
        Some(response),
        |client| {
            let result = client
                .scheduled_reports()
                .delete_scheduled_report(111, 999)?;
            assert_eq!(result, "OK");
            Ok(())
        },
    )
}
