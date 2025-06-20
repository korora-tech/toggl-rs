use crate::models::api::alert::{CreateAlert, UpdateAlert};
use crate::tests::*;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{AlertId, WorkspaceId};

#[test]
fn test_get_alerts() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([
        {
            "id": 201,
            "workspace_id": 12345,
            "alert_type": "weekly_time_limit",
            "threshold": 40.0,
            "enabled": true,
            "created_at": "2024-01-01T10:00:00Z",
            "updated_at": "2024-01-01T10:00:00Z",
            "meta": {
                "current_value": 35.5,
                "triggered": false
            }
        },
        {
            "id": 202,
            "workspace_id": 12345,
            "alert_type": "monthly_time_limit",
            "threshold": 160.0,
            "enabled": true,
            "created_at": "2024-01-02T10:00:00Z",
            "updated_at": "2024-01-02T10:00:00Z",
            "meta": {
                "current_value": 120.0,
                "triggered": false
            }
        }
    ]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/alerts", workspace_id),
        200,
        Some(response),
        |client| {
            let alerts = client.workspaces().get_alerts(workspace_id)?;
            assert_eq!(alerts.len(), 2);
            assert_eq!(alerts[0].alert.id, AlertId(201));
            assert_eq!(alerts[0].alert.threshold, 40.0);
            assert_eq!(alerts[0].alert.alert_type, "weekly_time_limit");
            assert_eq!(alerts[0].meta.current_value, 35.5);
            assert!(!alerts[0].meta.triggered);
            assert_eq!(alerts[1].alert.id, AlertId(202));
            assert_eq!(alerts[1].alert.threshold, 160.0);
            assert_eq!(alerts[1].alert.alert_type, "monthly_time_limit");
            Ok(())
        },
    )
}

#[test]
fn test_create_alert() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let new_alert = CreateAlert {
        alert_type: "weekly_time_limit".to_string(),
        threshold: 50.0,
        enabled: Some(true),
    };

    let response = json!({
        "id": 203,
        "workspace_id": 12345,
        "alert_type": "weekly_time_limit",
        "threshold": 50.0,
        "enabled": true,
        "created_at": "2024-01-17T15:00:00Z",
        "updated_at": "2024-01-17T15:00:00Z"
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/alerts", workspace_id),
        201,
        Some(response),
        |client| {
            let alert = client.workspaces().create_alert(workspace_id, &new_alert)?;
            assert_eq!(alert.id, AlertId(203));
            assert_eq!(alert.threshold, 50.0);
            assert_eq!(alert.alert_type, "weekly_time_limit");
            assert!(alert.enabled);
            Ok(())
        },
    )
}

#[test]
fn test_update_alert() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let alert_id = AlertId(201);

    let update = UpdateAlert {
        threshold: Some(45.0),
        enabled: Some(true),
    };

    let response = json!({
        "id": 201,
        "workspace_id": 12345,
        "alert_type": "weekly_time_limit",
        "threshold": 45.0,
        "enabled": true,
        "created_at": "2024-01-01T10:00:00Z",
        "updated_at": "2024-01-17T16:00:00Z"
    });

    with_mockito(
        Method::PUT,
        &format!("/workspaces/{}/alerts/{}", workspace_id, alert_id),
        200,
        Some(response),
        |client| {
            let alert = client
                .workspaces()
                .update_alert(workspace_id, alert_id, &update)?;
            assert_eq!(alert.id, alert_id);
            assert_eq!(alert.threshold, 45.0);
            assert!(alert.enabled);
            Ok(())
        },
    )
}

#[test]
fn test_delete_alert() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let alert_id = AlertId(201);

    with_mockito(
        Method::DELETE,
        &format!("/workspaces/{}/alerts/{}", workspace_id, alert_id),
        204,
        None,
        |client| {
            client.workspaces().delete_alert(workspace_id, alert_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_alerts() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/alerts", workspace_id),
        200,
        Some(response),
        |client| {
            let alerts = client.workspaces().get_alerts(workspace_id)?;
            assert!(alerts.is_empty());
            Ok(())
        },
    )
}

#[test]
fn test_create_minimal_alert() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let new_alert = CreateAlert {
        alert_type: "monthly_time_limit".to_string(),
        threshold: 80.0,
        enabled: Some(true),
    };

    let response = json!({
        "id": 204,
        "workspace_id": 12345,
        "alert_type": "monthly_time_limit",
        "threshold": 80.0,
        "enabled": true,
        "created_at": "2024-01-17T17:00:00Z",
        "updated_at": "2024-01-17T17:00:00Z"
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/alerts", workspace_id),
        201,
        Some(response),
        |client| {
            let alert = client.workspaces().create_alert(workspace_id, &new_alert)?;
            assert_eq!(alert.id, AlertId(204));
            assert_eq!(alert.threshold, 80.0);
            assert_eq!(alert.alert_type, "monthly_time_limit");
            assert!(alert.enabled);
            Ok(())
        },
    )
}
