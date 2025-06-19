use crate::models::api::shared_reports::{
    BulkDeleteRequest, CreateSavedReportPayload, SharedReportsQuery, UpdateSavedReportPayload,
};
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;
use reqwest::Method;

#[test]
fn test_get_shared_reports() -> Result<()> {
    let response = json!([
        {
            "id": 123,
            "name": "Weekly Report",
            "fixed_daterange": true,
            "public": true,
            "uid": 456,
            "updated_at": "2024-01-01T00:00:00Z",
            "updated_by": 456,
            "token": "abc123",
            "is_commenting_enabled": true
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/111/reports/shared",
        200,
        Some(response),
        |client| {
            let reports = client.shared_reports().get_shared_reports(111, None)?;
            assert_eq!(reports.len(), 1);
            assert_eq!(reports[0].name, Some("Weekly Report".to_string()));
            assert_eq!(reports[0].public, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_get_shared_reports_with_query() -> Result<()> {
    let response = json!([]);

    with_mockito(
        Method::GET,
        "/workspaces/111/reports/shared?fixed_dates=true&name=test&page=2&per_page=50&public=true",
        200,
        Some(response),
        |client| {
            let query = SharedReportsQuery {
                fixed_dates: Some(true),
                name: Some("test".to_string()),
                page: Some(2),
                per_page: Some(50),
                public: Some(true),
                requesting_user_id: None,
                scheduled: None,
                sort_direction: None,
                sort_field: None,
            };
            let reports = client
                .shared_reports()
                .get_shared_reports(111, Some(query))?;
            assert_eq!(reports.len(), 0);
            Ok(())
        },
    )
}

#[test]
fn test_create_shared_report() -> Result<()> {
    let request = CreateSavedReportPayload {
        name: "New Report".to_string(),
        fixed_daterange: Some(true),
        public: Some(false),
        params: None,
        regenerate_token: None,
    };

    let response = json!({
        "id": 124,
        "name": "New Report",
        "fixed_daterange": true,
        "public": false,
        "uid": 456,
        "updated_at": "2024-01-01T00:00:00Z",
        "token": "xyz789"
    });

    with_mockito(
        Method::POST,
        "/workspaces/111/reports/shared",
        200,
        Some(response),
        |client| {
            let report = client.shared_reports().create_shared_report(111, request)?;
            assert_eq!(report.id, Some(124));
            assert_eq!(report.name, Some("New Report".to_string()));
            assert_eq!(report.public, Some(false));
            Ok(())
        },
    )
}

#[test]
fn test_get_shared_report() -> Result<()> {
    let response = json!({
        "id": 123,
        "name": "Specific Report",
        "fixed_daterange": false,
        "public": true,
        "uid": 456,
        "params": "{\"period\":\"week\"}"
    });

    with_mockito(
        Method::GET,
        "/workspaces/111/reports/shared/123",
        200,
        Some(response),
        |client| {
            let report = client.shared_reports().get_shared_report(111, 123)?;
            assert_eq!(report.id, Some(123));
            assert_eq!(report.name, Some("Specific Report".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_update_shared_report() -> Result<()> {
    let request = UpdateSavedReportPayload {
        id: 123,
        name: Some("Updated Report".to_string()),
        fixed_daterange: Some(false),
        public: Some(true),
        params: None,
        regenerate_token: Some(true),
    };

    let response = json!({
        "id": 123,
        "name": "Updated Report",
        "fixed_daterange": false,
        "public": true,
        "uid": 456,
        "token": "newtoken123"
    });

    with_mockito(
        Method::PUT,
        "/workspaces/111/reports/shared/123",
        200,
        Some(response),
        |client| {
            let report = client
                .shared_reports()
                .update_shared_report(111, 123, request)?;
            assert_eq!(report.name, Some("Updated Report".to_string()));
            assert_eq!(report.public, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_delete_shared_report() -> Result<()> {
    let response = json!({
        "id": 123,
        "name": "Deleted Report",
        "deleted_at": "2024-01-01T00:00:00Z"
    });

    with_mockito(
        Method::DELETE,
        "/workspaces/111/reports/shared/123",
        200,
        Some(response),
        |client| {
            let report = client.shared_reports().delete_shared_report(111, 123)?;
            assert_eq!(report.id, Some(123));
            assert!(report.deleted_at.is_some());
            Ok(())
        },
    )
}

#[test]
fn test_bulk_delete_shared_reports() -> Result<()> {
    let request = BulkDeleteRequest {
        ids: vec![123, 124, 125],
    };

    let response = json!([
        {
            "id": 123,
            "name": "Report 1",
            "deleted_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 124,
            "name": "Report 2",
            "deleted_at": "2024-01-01T00:00:00Z"
        },
        {
            "id": 125,
            "name": "Report 3",
            "deleted_at": "2024-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::PATCH,
        "/workspaces/111/reports/shared/bulk_delete",
        200,
        Some(response),
        |client| {
            let reports = client
                .shared_reports()
                .bulk_delete_shared_reports(111, request)?;
            assert_eq!(reports.len(), 3);
            assert!(reports.iter().all(|r| r.deleted_at.is_some()));
            Ok(())
        },
    )
}
