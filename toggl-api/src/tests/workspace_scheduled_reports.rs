use crate::models::api::ids::{BookmarkId, ReportId, ScheduledReportId, UserId, WorkspaceId};
use crate::models::api::scheduled_reports::*;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

#[test]
fn test_get_scheduled_reports() -> Result<()> {
    let response = json!([
        {
            "report_id": 1,
            "bookmark_id": 100,
            "frequency": 7,
            "workspace_id": 987654,
            "creator_id": 123456,
            "created_at": "2024-01-01T10:00:00Z",
            "user_ids": [123456, 789012],
            "group_ids": [555]
        },
        {
            "report_id": 2,
            "bookmark_id": 101,
            "frequency": 30,
            "workspace_id": 987654,
            "creator_id": 123456,
            "created_at": "2024-01-15T14:00:00Z",
            "user_ids": [123456]
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/scheduled_reports",
        200,
        Some(response),
        |client| {
            let reports = client
                .workspace_reports()
                .get_scheduled_reports(WorkspaceId(987654))?;
            assert_eq!(reports.len(), 2);
            assert_eq!(reports[0].frequency, Some(7)); // Weekly
            assert_eq!(reports[1].frequency, Some(30)); // Monthly
            Ok(())
        },
    )
}

#[test]
fn test_create_scheduled_report() -> Result<()> {
    let response = json!({
        "report_id": 3,
        "bookmark_id": 102,
        "frequency": 1,
        "workspace_id": 987654,
        "creator_id": 123456,
        "created_at": "2024-01-20T12:00:00Z",
        "user_ids": [123456, 789012],
        "group_ids": []
    });

    with_mockito(
        Method::POST,
        "/workspaces/987654/scheduled_reports",
        200,
        Some(response),
        |client| {
            let report = CreateScheduledReportPayload {
                bookmark_id: BookmarkId(102),
                frequency: 1, // Daily
                user_ids: Some(vec![UserId(123456), UserId(789012)]),
                group_ids: None,
            };

            let created = client
                .workspace_reports()
                .create_scheduled_report(WorkspaceId(987654), &report)?;
            assert_eq!(created.bookmark_id, Some(BookmarkId(102)));
            assert_eq!(created.frequency, Some(1));
            assert_eq!(created.report_id, Some(ReportId(3)));
            Ok(())
        },
    )
}

#[test]
fn test_get_scheduled_report() -> Result<()> {
    let response = json!({
        "report_id": 4,
        "bookmark_id": 103,
        "frequency": 7,
        "workspace_id": 987654,
        "creator_id": 123456,
        "created_at": "2024-01-10T09:00:00Z",
        "user_ids": [123456, 456789],
        "group_ids": [777, 888]
    });

    with_mockito(
        Method::GET,
        "/workspaces/987654/scheduled_reports/4",
        200,
        Some(response),
        |client| {
            let report = client
                .workspace_reports()
                .get_scheduled_report(WorkspaceId(987654), ScheduledReportId(4))?;
            assert_eq!(report.report_id, Some(ReportId(4)));
            assert_eq!(report.frequency, Some(7)); // Weekly
            assert_eq!(report.group_ids.as_ref().unwrap().len(), 2);
            Ok(())
        },
    )
}

#[test]
fn test_delete_scheduled_report() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/987654/scheduled_reports/5",
        204,
        None,
        |client| {
            client
                .workspace_reports()
                .delete_scheduled_report(WorkspaceId(987654), ScheduledReportId(5))?;
            Ok(())
        },
    )
}
