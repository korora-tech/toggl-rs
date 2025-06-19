use crate::error::Result;
use crate::model::api::ids::{ProjectId, UserId};
use crate::tests::*;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_all_activity() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "activity": [
            {
                "user_id": 98765,
                "project_id": 54321,
                "duration": 3600,
                "description": "Working on feature X"
            },
            {
                "user_id": 98766,
                "project_id": 54322,
                "duration": 7200,
                "description": "Code review"
            }
        ]
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/all_activity", workspace_id),
        200,
        Some(response),
        |client| {
            let activity = client
                .workspaces()
                .get_dashboard_all_activity(workspace_id, None)?;
            assert_eq!(activity.activity.len(), 2);
            assert_eq!(activity.activity[0].user_id, UserId(98765));
            assert_eq!(activity.activity[0].duration, 3600);
            assert_eq!(
                activity.activity[1].description,
                Some("Code review".to_string())
            );
            Ok(())
        },
    )
}

#[test]
fn test_get_most_active() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "most_active": [
            {
                "user_id": 98765,
                "duration": 28800
            },
            {
                "user_id": 98766,
                "duration": 14400
            }
        ]
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/most_active", workspace_id),
        200,
        Some(response),
        |client| {
            let most_active = client
                .workspaces()
                .get_dashboard_most_active(workspace_id, None)?;
            assert_eq!(most_active.most_active.len(), 2);
            assert_eq!(most_active.most_active[0].user_id, UserId(98765));
            assert_eq!(most_active.most_active[0].duration, 28800);
            assert_eq!(most_active.most_active[1].user_id, UserId(98766));
            assert_eq!(most_active.most_active[1].duration, 14400);
            Ok(())
        },
    )
}

#[test]
fn test_get_top_activity() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "top_activity": [
            {
                "project_id": 54321,
                "duration": 57600,
                "user_count": 25
            },
            {
                "project_id": 54322,
                "duration": 28800,
                "user_count": 15
            },
            {
                "project_id": 54323,
                "duration": 14400,
                "user_count": 10
            }
        ]
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/top_activity", workspace_id),
        200,
        Some(response),
        |client| {
            let top_activity = client
                .workspaces()
                .get_dashboard_top_activity(workspace_id, None)?;
            assert_eq!(top_activity.top_activity.len(), 3);
            assert_eq!(
                top_activity.top_activity[0].project_id,
                Some(ProjectId(54321))
            );
            assert_eq!(top_activity.top_activity[0].duration, 57600);
            assert_eq!(top_activity.top_activity[0].user_count, 25);
            assert_eq!(
                top_activity.top_activity[1].project_id,
                Some(ProjectId(54322))
            );
            assert_eq!(top_activity.top_activity[2].user_count, 10);
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_all_activity_dashboard() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "activity": []
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/all_activity", workspace_id),
        200,
        Some(response),
        |client| {
            let activity = client
                .workspaces()
                .get_dashboard_all_activity(workspace_id, None)?;
            assert!(activity.activity.is_empty());
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_most_active_dashboard() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "most_active": []
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/most_active", workspace_id),
        200,
        Some(response),
        |client| {
            let most_active = client
                .workspaces()
                .get_dashboard_most_active(workspace_id, None)?;
            assert!(most_active.most_active.is_empty());
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_top_activity_dashboard() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "top_activity": []
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/dashboard/top_activity", workspace_id),
        200,
        Some(response),
        |client| {
            let top_activity = client
                .workspaces()
                .get_dashboard_top_activity(workspace_id, None)?;
            assert!(top_activity.top_activity.is_empty());
            Ok(())
        },
    )
}
