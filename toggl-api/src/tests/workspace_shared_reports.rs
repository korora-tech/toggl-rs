use crate::models::api::shared_reports::*;
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use std::collections::HashMap;
use toggl_core::Result;
use toggl_core::{ReportId, WorkspaceId};

#[test]
fn test_get_shared_reports() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "name": "Weekly Team Report",
            "fixed_daterange": true,
            "public": false,
            "params": "{\"billable\":true,\"client_ids\":[123]}",
            "token": "abcd1234",
            "uid": 123456,
            "updated_at": "2024-01-15T10:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/reports/shared",
        200,
        Some(response),
        |client| {
            let reports = client
                .workspace_reports()
                .get_shared_reports(WorkspaceId(987654), None)?;
            assert_eq!(reports.len(), 1);
            assert_eq!(reports[0].name.as_ref().unwrap(), "Weekly Team Report");
            Ok(())
        },
    )
}

#[test]
fn test_get_shared_reports_with_query() -> Result<()> {
    let response = json!([
        {
            "id": 2,
            "name": "Public Dashboard",
            "fixed_daterange": false,
            "public": true,
            "params": "{}",
            "token": "xyz789",
            "uid": 123456
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/reports/shared?public=true&sort_field=name",
        200,
        Some(response),
        |client| {
            let query = SharedReportsQuery {
                public: Some(true),
                sort_field: Some("name".to_string()),
                ..Default::default()
            };
            let reports = client
                .workspace_reports()
                .get_shared_reports(WorkspaceId(987654), Some(&query))?;
            assert_eq!(reports.len(), 1);
            assert_eq!(reports[0].public, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_create_shared_report() -> Result<()> {
    let response = json!({
        "id": 3,
        "name": "Monthly Revenue Report",
        "fixed_daterange": true,
        "public": false,
        "params": "{\"project_ids\":[456,789]}",
        "token": "newtoken123",
        "uid": 123456,
        "updated_at": "2024-01-20T15:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/987654/reports/shared",
        200,
        Some(response),
        |client| {
            let mut params = HashMap::new();
            params.insert("project_ids".to_string(), json!([456, 789]));

            let report = CreateSavedReportPayload {
                name: "Monthly Revenue Report".to_string(),
                fixed_daterange: Some(true),
                public: Some(false),
                params: Some(params),
                regenerate_token: None,
            };

            let created = client
                .workspace_reports()
                .create_shared_report(WorkspaceId(987654), &report)?;
            assert_eq!(created.name.as_ref().unwrap(), "Monthly Revenue Report");
            assert_eq!(created.id, Some(3));
            Ok(())
        },
    )
}

#[test]
fn test_update_shared_reports() -> Result<()> {
    let response = json!({
        "id": 4,
        "name": "Updated Report Name",
        "fixed_daterange": false,
        "public": true,
        "token": "updatedtoken",
        "uid": 123456
    });

    with_mockito(
        Method::PUT,
        "/workspaces/987654/reports/shared",
        200,
        Some(response),
        |client| {
            let reports = vec![UpdateSavedReportPayload {
                id: 4,
                name: Some("Updated Report Name".to_string()),
                fixed_daterange: Some(false),
                public: Some(true),
                params: None,
                regenerate_token: Some(true),
            }];

            let updated = client
                .workspace_reports()
                .update_shared_reports(WorkspaceId(987654), &reports)?;
            assert_eq!(updated.name.as_ref().unwrap(), "Updated Report Name");
            Ok(())
        },
    )
}

#[test]
fn test_get_shared_report() -> Result<()> {
    let response = json!({
        "id": 5,
        "name": "Specific Report",
        "fixed_daterange": true,
        "public": false,
        "params": "{}",
        "token": "specific123",
        "uid": 123456,
        "is_commenting_enabled": true
    });

    with_mockito(
        Method::GET,
        "/workspaces/987654/reports/shared/5",
        200,
        Some(response),
        |client| {
            let report = client
                .workspace_reports()
                .get_shared_report(WorkspaceId(987654), ReportId(5))?;
            assert_eq!(report.id, Some(5));
            assert_eq!(report.name.as_ref().unwrap(), "Specific Report");
            assert_eq!(report.is_commenting_enabled, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_update_shared_report() -> Result<()> {
    let response = json!({
        "id": 6,
        "name": "Updated Single Report",
        "fixed_daterange": true,
        "public": false,
        "token": "updated456",
        "uid": 123456
    });

    with_mockito(
        Method::PUT,
        "/workspaces/987654/reports/shared/6",
        200,
        Some(response),
        |client| {
            let update = UpdateSavedReportPayload {
                id: 6,
                name: Some("Updated Single Report".to_string()),
                fixed_daterange: Some(true),
                public: Some(false),
                params: None,
                regenerate_token: None,
            };

            let updated = client.workspace_reports().update_shared_report(
                WorkspaceId(987654),
                ReportId(6),
                &update,
            )?;
            assert_eq!(updated.name.as_ref().unwrap(), "Updated Single Report");
            Ok(())
        },
    )
}

#[test]
fn test_delete_shared_report() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/987654/reports/shared/7",
        204,
        None,
        |client| {
            client
                .workspace_reports()
                .delete_shared_report(WorkspaceId(987654), ReportId(7))?;
            Ok(())
        },
    )
}

#[test]
fn test_bulk_delete_shared_reports() -> Result<()> {
    with_mockito(
        Method::PATCH,
        "/workspaces/987654/reports/shared/bulk_delete",
        204,
        None,
        |client| {
            client.workspace_reports().bulk_delete_shared_reports(
                WorkspaceId(987654),
                vec![ReportId(8), ReportId(9), ReportId(10)],
            )?;
            Ok(())
        },
    )
}
