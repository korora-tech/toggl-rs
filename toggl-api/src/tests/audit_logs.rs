use crate::client::audit::AuditLogFilters;
use crate::tests::*;
use chrono::{DateTime, Utc};
use reqwest::Method;
use serde_json::json;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{OrganizationId, UserId, WorkspaceId};

#[test]
fn test_get_audit_logs() -> Result<()> {
    let org_id = OrganizationId(12345);
    let from = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let to = DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    let response = json!([
        {
            "id": "log_123",
            "timestamp": "2024-01-15T10:30:00Z",
            "user_id": 98765,
            "event_type": "workspace.created",
            "details": {
                "workspace_id": 54321,
                "workspace_name": "Test Workspace"
            }
        },
        {
            "id": "log_124",
            "timestamp": "2024-01-16T14:20:00Z",
            "user_id": 98766,
            "event_type": "project.updated",
            "details": {
                "project_id": 11111,
                "changes": {
                    "name": {
                        "old": "Old Name",
                        "new": "New Name"
                    }
                }
            }
        }
    ]);

    with_mockito(
        Method::GET,
        &format!(
            "/audit_logs/{}/{}/{}",
            org_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        ),
        200,
        Some(response),
        |client| {
            let logs = client.audit().get_logs(org_id, from, to)?;
            assert_eq!(logs.len(), 2);
            assert_eq!(logs[0].id, "log_123".into());
            assert_eq!(logs[0].user_id, 98765.into());
            assert_eq!(logs[0].event_type, "workspace.created");
            assert_eq!(logs[1].id, "log_124".into());
            assert_eq!(logs[1].event_type, "project.updated");
            Ok(())
        },
    )
}

#[test]
fn test_get_audit_logs_with_filters() -> Result<()> {
    let org_id = OrganizationId(12345);
    let from = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let to = DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    let response = json!([
        {
            "id": "log_125",
            "timestamp": "2024-01-20T09:00:00Z",
            "user_id": 98765,
            "event_type": "time_entry.created",
            "details": {
                "time_entry_id": 33333,
                "workspace_id": 54321
            }
        }
    ]);

    let mut params = BTreeMap::new();
    params.insert("workspace_id", "54321");
    params.insert("entity_type", "time_entry");
    params.insert("action", "created");
    params.insert("user_id", "98765");

    with_mockito_params(
        Method::GET,
        &format!(
            "/audit_logs/{}/{}/{}",
            org_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        ),
        Some(params),
        200,
        Some(response),
        |client| {
            let filters = AuditLogFilters {
                workspace_id: Some(WorkspaceId(54321)),
                entity_type: Some("time_entry".to_string()),
                entity_id: None,
                action: Some("created".to_string()),
                user_id: Some(UserId(98765)),
                page_size: None,
                offset: None,
            };
            let logs = client
                .audit()
                .get_logs_with_filters(org_id, from, to, &filters)?;
            assert_eq!(logs.len(), 1);
            assert_eq!(logs[0].event_type, "time_entry.created");
            Ok(())
        },
    )
}

#[test]
fn test_get_audit_logs_export() -> Result<()> {
    let org_id = OrganizationId(12345);
    let from = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let to = DateTime::parse_from_rfc3339("2024-12-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    // Export returns all logs without pagination
    let response = json!([
        {
            "id": "log_001",
            "timestamp": "2024-01-01T00:00:00Z",
            "user_id": 98765,
            "event_type": "workspace.created",
            "details": {}
        },
        // ... many more logs
    ]);

    let mut params = BTreeMap::new();
    params.insert("export", "true");

    with_mockito_params(
        Method::GET,
        &format!(
            "/audit_logs/{}/{}/{}",
            org_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        ),
        Some(params),
        200,
        Some(response),
        |client| {
            let logs = client.audit().export_logs(org_id, from, to)?;
            assert!(!logs.is_empty());
            Ok(())
        },
    )
}

#[test]
fn test_get_audit_logs_pagination() -> Result<()> {
    let org_id = OrganizationId(12345);
    let from = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let to = DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
        .unwrap()
        .with_timezone(&Utc);

    let response = json!([
        {
            "id": "log_page1_001",
            "timestamp": "2024-01-01T00:00:00Z",
            "user_id": 98765,
            "event_type": "workspace.created",
            "details": {}
        }
    ]);

    let mut params = BTreeMap::new();
    params.insert("page_size", "50");

    with_mockito_params(
        Method::GET,
        &format!(
            "/audit_logs/{}/{}/{}",
            org_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        ),
        Some(params),
        200,
        Some(response),
        |client| {
            let logs = client.audit().get_logs_paginated(org_id, from, to, 50)?;
            assert_eq!(logs.len(), 1);
            Ok(())
        },
    )
}

#[test]
fn test_get_audit_logs_empty() -> Result<()> {
    let org_id = OrganizationId(12345);
    let from = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);
    let to = DateTime::parse_from_rfc3339("2024-01-02T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc);

    let response = json!([]);

    with_mockito(
        Method::GET,
        &format!(
            "/audit_logs/{}/{}/{}",
            org_id,
            from.to_rfc3339(),
            to.to_rfc3339()
        ),
        200,
        Some(response),
        |client| {
            let logs = client.audit().get_logs(org_id, from, to)?;
            assert!(logs.is_empty());
            Ok(())
        },
    )
}
