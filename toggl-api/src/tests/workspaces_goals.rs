use crate::models::api::goals::{CreateGoalRequest, UpdateGoalRequest, WorkspaceGoalsQuery};
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{GoalId, TagId, UserId, WorkspaceId};

#[test]
fn test_get_workspace_goals() -> Result<()> {
    let response = json!([
        {
            "goal_id": 1,
            "active": true,
            "billable": false,
            "comparison": ">=",
            "creator_user_id": 123456,
            "creator_user_name": "John Doe",
            "end_date": "2024-12-31",
            "icon": "🎯",
            "name": "Daily coding goal",
            "permissions": ["edit", "delete"],
            "project_ids": [789],
            "recurrence": "daily",
            "start_date": "2024-01-01",
            "status": "active",
            "streak": 5,
            "tag_ids": [456],
            "tags": ["work"],
            "target_seconds": 28800,
            "task_ids": [],
            "team_goal": false,
            "user_id": 123456,
            "user_name": "John Doe",
            "workspace_id": 987654
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/goals",
        200,
        Some(response),
        |client| {
            let goals = client.workspaces().get_goals(WorkspaceId(987654), None)?;
            assert_eq!(goals.len(), 1);
            assert_eq!(goals[0].name, "Daily coding goal");
            assert_eq!(goals[0].target_seconds, 28800);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_goals_with_query() -> Result<()> {
    let response = json!([
        {
            "goal_id": 2,
            "active": true,
            "billable": true,
            "comparison": ">=",
            "creator_user_id": 123456,
            "creator_user_name": "John Doe",
            "end_date": "2024-12-31",
            "icon": "💼",
            "name": "Team billable hours",
            "permissions": ["edit", "delete"],
            "project_ids": [789, 790],
            "recurrence": "weekly",
            "start_date": "2024-01-01",
            "status": "active",
            "streak": 12,
            "tag_ids": [],
            "tags": [],
            "target_seconds": 144000,
            "task_ids": [],
            "team_goal": true,
            "user_id": 123456,
            "user_name": "John Doe",
            "workspace_id": 987654
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/goals?active=true&team_goals=true",
        200,
        Some(response),
        |client| {
            let query = WorkspaceGoalsQuery {
                team_goals: Some(true),
                active: Some(true),
                ..Default::default()
            };
            let goals = client
                .workspaces()
                .get_goals(WorkspaceId(987654), Some(&query))?;
            assert_eq!(goals.len(), 1);
            assert_eq!(goals[0].name, "Team billable hours");
            assert!(goals[0].team_goal);
            Ok(())
        },
    )
}

#[test]
fn test_create_workspace_goal() -> Result<()> {
    let response = json!({
        "id": 3,
        "active": true,
        "billable": false,
        "comparison": ">=",
        "creatorUserID": 123456,
        "creatorUserName": "John Doe",
        "endDate": "2024-12-31",
        "icon": "📚",
        "name": "Study time",
        "permissions": ["edit", "delete"],
        "projectIDs": [],
        "recurrence": "daily",
        "startDate": "2024-01-01",
        "status": "active",
        "streak": 0,
        "tagIDs": [123],
        "tags": ["learning"],
        "targetSeconds": 7200,
        "taskIDs": [],
        "teamGoal": false,
        "userID": 123456,
        "userName": "John Doe",
        "workspaceID": 987654
    });

    with_mockito(
        Method::POST,
        "/workspaces/987654/goals",
        200,
        Some(response),
        |client| {
            let goal = CreateGoalRequest {
                billable: false,
                comparison: ">=".to_string(),
                end_date: "2024-12-31".to_string(),
                icon: "📚".to_string(),
                name: "Study time".to_string(),
                project_ids: vec![],
                recurrence: "daily".to_string(),
                start_date: "2024-01-01".to_string(),
                tag_ids: vec![TagId(123)],
                target_seconds: 7200,
                task_ids: vec![],
                user_id: UserId(123456),
            };

            let created = client
                .workspaces()
                .create_goal(WorkspaceId(987654), &goal)?;
            assert_eq!(created.id, GoalId(3));
            assert_eq!(created.name, "Study time");
            assert_eq!(created.target_seconds, 7200);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_goal() -> Result<()> {
    let response = json!({
        "goal_id": 4,
        "active": true,
        "billable": true,
        "comparison": ">=",
        "creator_user_id": 123456,
        "creator_user_name": "Jane Smith",
        "end_date": "2024-06-30",
        "icon": "💰",
        "name": "Q2 Revenue Goal",
        "permissions": ["view"],
        "project_ids": [100, 101, 102],
        "recurrence": "quarterly",
        "start_date": "2024-04-01",
        "status": "active",
        "streak": 1,
        "tag_ids": [],
        "tags": [],
        "target_seconds": 432000,
        "task_ids": [],
        "team_goal": true,
        "user_id": 123456,
        "user_name": "Jane Smith",
        "workspace_id": 987654
    });

    with_mockito(
        Method::GET,
        "/workspaces/987654/goals/4",
        200,
        Some(response),
        |client| {
            let goal = client
                .workspaces()
                .get_goal(WorkspaceId(987654), GoalId(4))?;
            assert_eq!(goal.id, GoalId(4));
            assert_eq!(goal.name, "Q2 Revenue Goal");
            assert!(goal.team_goal);
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace_goal() -> Result<()> {
    let response = json!({
        "id": 5,
        "active": false,
        "billable": false,
        "comparison": ">=",
        "creatorUserID": 123456,
        "creatorUserName": "John Doe",
        "endDate": "2024-12-31",
        "icon": "✅",
        "name": "Completed Goal",
        "permissions": ["edit", "delete"],
        "projectIDs": [200],
        "recurrence": "none",
        "startDate": "2024-01-01",
        "status": "completed",
        "streak": 30,
        "tagIDs": [],
        "tags": [],
        "targetSeconds": 14400,
        "taskIDs": [],
        "teamGoal": false,
        "userID": 123456,
        "userName": "John Doe",
        "workspaceID": 987654
    });

    with_mockito(
        Method::PUT,
        "/workspaces/987654/goals/5",
        200,
        Some(response),
        |client| {
            let update = UpdateGoalRequest {
                active: false,
                comparison: ">=".to_string(),
                end_date: "2024-12-31".to_string(),
                icon: "✅".to_string(),
                last_notified_at: None,
                name: "Completed Goal".to_string(),
                target_seconds: 14400,
            };

            let updated =
                client
                    .workspaces()
                    .update_goal(WorkspaceId(987654), GoalId(5), &update)?;
            assert_eq!(updated.id, GoalId(5));
            assert_eq!(updated.name, "Completed Goal");
            assert!(!updated.active);
            Ok(())
        },
    )
}

#[test]
fn test_delete_workspace_goal() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/987654/goals/6",
        204,
        None,
        |client| {
            client
                .workspaces()
                .delete_goal(WorkspaceId(987654), GoalId(6))?;
            Ok(())
        },
    )
}
