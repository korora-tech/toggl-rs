use crate::models::api::goals::{CreateGoalRequest, UpdateGoalRequest, WorkspaceGoalsQuery};
use serde_json::json;
use toggl_core::Result;
use toggl_core::{GoalId, ProjectId, TagId, UserId, WorkspaceId};

use super::with_mockito;
use reqwest::Method;

#[test]
fn test_get_workspace_goals() -> Result<()> {
    let response = json!([
        {
            "goal_id": 123,
            "active": true,
            "billable": false,
            "comparison": "at_least",
            "creator_user_id": 456,
            "creator_user_name": "John Doe",
            "current_recurrence_end_date": "2024-01-31",
            "current_recurrence_start_date": "2024-01-01",
            "current_recurrence_tracked_seconds": 3600,
            "end_date": "2024-12-31",
            "icon": "🎯",
            "last_completed_recurrence_end_date": "2023-12-31",
            "last_notified_at": "2024-01-15T10:00:00Z",
            "name": "Weekly coding goal",
            "permissions": ["read", "write"],
            "project_ids": [789, 790],
            "recurrence": "weekly",
            "start_date": "2024-01-01",
            "status": "active",
            "streak": 5,
            "tag_ids": [101, 102],
            "tags": ["coding", "development"],
            "target_seconds": 14400,
            "task_ids": [201, 202],
            "team_goal": false,
            "user_id": 456,
            "user_name": "John Doe",
            "workspace_id": 111
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/111/goals",
        200,
        Some(response),
        |client| {
            let goals = client.goals().get_workspace_goals(WorkspaceId(111), None)?;
            assert_eq!(goals.len(), 1);

            let goal = &goals[0];
            assert_eq!(goal.id, GoalId(123));
            assert_eq!(goal.name, "Weekly coding goal");
            assert_eq!(goal.comparison, "at_least");
            assert_eq!(goal.target_seconds, 14400);
            assert_eq!(goal.recurrence, "weekly");
            assert!(!goal.team_goal);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_goals_with_query() -> Result<()> {
    let response = json!([]);

    with_mockito(
        Method::GET,
        "/workspaces/111/goals?active=true&page=2&per_page=50&team_goals=true",
        200,
        Some(response),
        |client| {
            let query = WorkspaceGoalsQuery {
                team_goals: Some(true),
                active: Some(true),
                page: Some(2),
                per_page: Some(50),
            };
            let goals = client
                .goals()
                .get_workspace_goals(WorkspaceId(111), Some(query))?;
            assert_eq!(goals.len(), 0);
            Ok(())
        },
    )
}

#[test]
fn test_create_workspace_goal() -> Result<()> {
    let request = CreateGoalRequest {
        billable: false,
        comparison: "at_least".to_string(),
        end_date: "2024-12-31".to_string(),
        icon: "🎯".to_string(),
        name: "New Goal".to_string(),
        project_ids: vec![ProjectId(789)],
        recurrence: "weekly".to_string(),
        start_date: "2024-01-01".to_string(),
        tag_ids: vec![TagId(101)],
        target_seconds: 14400,
        task_ids: vec![],
        user_id: UserId(456),
    };

    let response = json!({
        "id": 124,
        "active": true,
        "billable": false,
        "comparison": "at_least",
        "creatorUserID": 456,
        "creatorUserName": "John Doe",
        "currentRecurrenceEndDate": "2024-01-31",
        "currentRecurrenceStartDate": "2024-01-01",
        "currentRecurrenceTrackedSeconds": 0,
        "endDate": "2024-12-31",
        "icon": "🎯",
        "name": "New Goal",
        "permissions": ["read", "write"],
        "projectIDs": [789],
        "recurrence": "weekly",
        "startDate": "2024-01-01",
        "status": "active",
        "streak": 0,
        "tagIDs": [101],
        "tags": ["coding"],
        "targetSeconds": 14400,
        "taskIDs": [],
        "teamGoal": false,
        "userID": 456,
        "userName": "John Doe",
        "workspaceID": 111
    });

    with_mockito(
        Method::POST,
        "/workspaces/111/goals",
        200,
        Some(response),
        |client| {
            let goal = client
                .goals()
                .create_workspace_goal(WorkspaceId(111), request)?;
            assert_eq!(goal.id, GoalId(124));
            assert_eq!(goal.name, "New Goal");
            assert_eq!(goal.target_seconds, 14400);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_goal() -> Result<()> {
    let response = json!({
        "goal_id": 123,
        "active": true,
        "billable": false,
        "comparison": "at_least",
        "creator_user_id": 456,
        "creator_user_name": "John Doe",
        "end_date": "2024-12-31",
        "icon": "🎯",
        "name": "Weekly coding goal",
        "permissions": ["read", "write"],
        "project_ids": [789],
        "recurrence": "weekly",
        "start_date": "2024-01-01",
        "status": "active",
        "streak": 5,
        "tag_ids": [101],
        "tags": ["coding"],
        "target_seconds": 14400,
        "task_ids": [],
        "team_goal": false,
        "user_id": 456,
        "user_name": "John Doe",
        "workspace_id": 111
    });

    with_mockito(
        Method::GET,
        "/workspaces/111/goals/123",
        200,
        Some(response),
        |client| {
            let goal = client
                .goals()
                .get_workspace_goal(WorkspaceId(111), GoalId(123))?;
            assert_eq!(goal.id, GoalId(123));
            assert_eq!(goal.name, "Weekly coding goal");
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace_goal() -> Result<()> {
    let request = UpdateGoalRequest {
        active: false,
        comparison: "at_most".to_string(),
        end_date: "2024-06-30".to_string(),
        icon: "✅".to_string(),
        last_notified_at: None,
        name: "Updated Goal".to_string(),
        target_seconds: 7200,
    };

    let response = json!({
        "id": 123,
        "active": false,
        "billable": false,
        "comparison": "at_most",
        "creatorUserID": 456,
        "creatorUserName": "John Doe",
        "endDate": "2024-06-30",
        "icon": "✅",
        "name": "Updated Goal",
        "permissions": ["read", "write"],
        "projectIDs": [789],
        "recurrence": "weekly",
        "startDate": "2024-01-01",
        "status": "inactive",
        "streak": 5,
        "tagIDs": [101],
        "tags": ["coding"],
        "targetSeconds": 7200,
        "taskIDs": [],
        "teamGoal": false,
        "userID": 456,
        "userName": "John Doe",
        "workspaceID": 111
    });

    with_mockito(
        Method::PUT,
        "/workspaces/111/goals/123",
        200,
        Some(response),
        |client| {
            let goal =
                client
                    .goals()
                    .update_workspace_goal(WorkspaceId(111), GoalId(123), request)?;
            assert_eq!(goal.id, GoalId(123));
            assert_eq!(goal.name, "Updated Goal");
            assert_eq!(goal.target_seconds, 7200);
            assert!(!goal.active);
            Ok(())
        },
    )
}

#[test]
fn test_delete_workspace_goal() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/111/goals/123",
        204,
        None,
        |client| {
            client
                .goals()
                .delete_workspace_goal(WorkspaceId(111), GoalId(123))?;
            Ok(())
        },
    )
}
