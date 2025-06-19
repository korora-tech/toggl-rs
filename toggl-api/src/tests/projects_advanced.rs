use crate::models::api::project::*;
use crate::tests::{with_mockito, with_mockito_params};
use reqwest::Method;
use serde_json::json;
use std::collections::BTreeMap;
use toggl_core::Result;

#[test]
fn test_bulk_edit_projects() -> Result<()> {
    let operations = vec![
        PatchOperation {
            op: "replace".to_string(),
            path: "/color".to_string(),
            value: Some(json!("#FF0000")),
        },
        PatchOperation {
            op: "replace".to_string(),
            path: "/active".to_string(),
            value: Some(json!(true)),
        },
    ];

    let response = json!([
        {
            "id": 123,
            "workspace_id": 1,
            "name": "Project 1",
            "color": "#FF0000",
            "active": true,
            "is_private": false,
            "at": "2023-01-01T00:00:00Z",
            "created_at": "2023-01-01T00:00:00Z",
            "recurring": false
        },
        {
            "id": 124,
            "workspace_id": 1,
            "name": "Project 2",
            "color": "#FF0000",
            "active": true,
            "is_private": false,
            "at": "2023-01-01T00:00:00Z",
            "created_at": "2023-01-01T00:00:00Z",
            "recurring": false
        }
    ]);

    with_mockito(
        Method::PATCH,
        "/workspaces/1/projects/123,124",
        200,
        Some(response),
        |client| {
            let projects = client.projects().bulk_edit(1, &[123, 124], &operations)?;
            assert_eq!(projects.len(), 2);
            assert_eq!(projects[0].color, "#FF0000");
            assert!(projects[0].active);
            Ok(())
        },
    )
}

#[test]
fn test_get_project_periods() -> Result<()> {
    let response = json!([
        {
            "start_date": "2023-01-01",
            "end_date": "2023-01-31"
        },
        {
            "start_date": "2023-02-01",
            "end_date": "2023-02-28"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1/projects/123/periods",
        200,
        Some(response),
        |client| {
            let periods = client.projects().get_periods(1, 123, None, None)?;
            assert_eq!(periods.len(), 2);
            Ok(())
        },
    )
}

#[test]
fn test_get_project_periods_with_dates() -> Result<()> {
    let response = json!([
        {
            "start_date": "2023-01-01",
            "end_date": "2023-01-31"
        }
    ]);

    let mut params = BTreeMap::new();
    params.insert("start_date", "2023-01-01");
    params.insert("end_date", "2023-12-31");

    with_mockito_params(
        Method::GET,
        "/workspaces/1/projects/123/periods",
        Some(params),
        200,
        Some(response),
        |client| {
            let periods =
                client
                    .projects()
                    .get_periods(1, 123, Some("2023-01-01"), Some("2023-12-31"))?;
            assert_eq!(periods.len(), 1);
            Ok(())
        },
    )
}

#[test]
fn test_pin_project() -> Result<()> {
    with_mockito(
        Method::POST,
        "/workspaces/1/projects/123/pin",
        204,
        None,
        |client| {
            client.projects().pin(1, 123)?;
            Ok(())
        },
    )
}

#[test]
fn test_unpin_project() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1/projects/123/pin",
        204,
        None,
        |client| {
            client.projects().unpin(1, 123)?;
            Ok(())
        },
    )
}

#[test]
fn test_bulk_edit_tasks() -> Result<()> {
    let operations = vec![PatchOperation {
        op: "replace".to_string(),
        path: "/active".to_string(),
        value: Some(json!(false)),
    }];

    let response = json!([
        {
            "id": 456,
            "name": "Task 1",
            "active": false,
            "project_id": 123,
            "workspace_id": 1,
            "recurring": false,
            "at": "2023-01-01T00:00:00Z"
        },
        {
            "id": 457,
            "name": "Task 2",
            "active": false,
            "project_id": 123,
            "workspace_id": 1,
            "recurring": false,
            "at": "2023-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::PATCH,
        "/workspaces/1/projects/123/tasks/456,457",
        200,
        Some(response),
        |client| {
            let tasks = client
                .projects()
                .bulk_edit_tasks(1, 123, &[456, 457], &operations)?;
            assert_eq!(tasks.len(), 2);
            assert!(!tasks[0].active);
            Ok(())
        },
    )
}

#[test]
fn test_get_billable_amounts() -> Result<()> {
    let response = json!([
        {
            "id": 123,
            "workspace_id": 1,
            "name": "Project 1",
            "billable": true,
            "actual_seconds": 3600,
            "rate": 100.0,
            "currency": "USD",
            "is_private": false,
            "active": true,
            "at": "2023-01-01T00:00:00Z",
            "created_at": "2023-01-01T00:00:00Z",
            "color": "#0000FF",
            "recurring": false
        }
    ]);

    with_mockito(
        Method::POST,
        "/workspaces/1/projects/billable-amounts",
        200,
        Some(response),
        |client| {
            let projects = client.projects().get_billable_amounts(1, &[123, 124])?;
            assert_eq!(projects.len(), 1);
            assert_eq!(projects[0].billable, Some(true));
            Ok(())
        },
    )
}

#[test]
fn test_get_task_count() -> Result<()> {
    let response = json!({
        "total": 42,
        "by_project": {
            "123": 10,
            "124": 32
        }
    });

    with_mockito(
        Method::GET,
        "/workspaces/1/projects/task_count",
        200,
        Some(response),
        |client| {
            let count = client.projects().get_task_count(1)?;
            assert_eq!(count["total"], 42);
            Ok(())
        },
    )
}

#[test]
fn test_get_templates() -> Result<()> {
    let response = json!([
        {
            "id": 123,
            "name": "Template 1",
            "active": true,
            "billable": true,
            "is_private": false,
            "color": "#0000FF",
            "auto_estimates": false,
            "estimated_hours": 10,
            "rate": 100.0
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1/projects/templates",
        200,
        Some(response),
        |client| {
            let templates = client.projects().get_templates(1)?;
            assert_eq!(templates.len(), 1);
            assert_eq!(templates[0].name, "Template 1");
            Ok(())
        },
    )
}

#[test]
fn test_get_user_count() -> Result<()> {
    let response = json!({
        "total": 15,
        "by_project": {
            "123": 5,
            "124": 10
        }
    });

    with_mockito(
        Method::GET,
        "/workspaces/1/projects/user_count",
        200,
        Some(response),
        |client| {
            let count = client.projects().get_user_count(1)?;
            assert_eq!(count["total"], 15);
            Ok(())
        },
    )
}

#[test]
fn test_get_project_groups() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "group_id": 10,
            "project_id": 123,
            "workspace_id": 1
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1/project_groups",
        200,
        Some(response),
        |client| {
            let groups = client.projects().get_groups(1)?;
            assert_eq!(groups.len(), 1);
            assert_eq!(groups[0].group_id, 10);
            Ok(())
        },
    )
}

#[test]
fn test_create_project_group() -> Result<()> {
    let payload = ProjectGroupPayload {
        project_id: 123,
        group_id: 10,
    };

    let response = json!({
        "id": 1,
        "group_id": 10,
        "project_id": 123,
        "workspace_id": 1
    });

    with_mockito(
        Method::POST,
        "/workspaces/1/project_groups",
        200,
        Some(response),
        |client| {
            let group = client.projects().create_group(1, &payload)?;
            assert_eq!(group.group_id, 10);
            assert_eq!(group.project_id, 123);
            Ok(())
        },
    )
}

#[test]
fn test_delete_project_group() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1/project_groups/1",
        204,
        None,
        |client| {
            client.projects().delete_group(1, 1)?;
            Ok(())
        },
    )
}

#[test]
fn test_list_project_users() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "project_id": 123,
            "user_id": 456,
            "workspace_id": 1,
            "manager": true,
            "rate": 100.0,
            "at": "2023-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1/project_users",
        200,
        Some(response),
        |client| {
            let users = client.projects().list_project_users(1)?;
            assert_eq!(users.len(), 1);
            assert!(users[0].manager);
            Ok(())
        },
    )
}

#[test]
fn test_list_project_users_paginated() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "project_id": 123,
            "user_id": 456,
            "workspace_id": 1,
            "manager": true,
            "rate": 100.0,
            "at": "2023-01-01T00:00:00Z"
        }
    ]);

    let mut params = BTreeMap::new();
    params.insert("page", "1");
    params.insert("per_page", "10");
    params.insert("sort_field", "name");
    params.insert("sort_order", "asc");

    with_mockito_params(
        Method::GET,
        "/workspaces/1/project_users/paginated",
        Some(params),
        200,
        Some(response),
        |client| {
            let users = client.projects().list_project_users_paginated(
                1,
                Some(1),
                Some(10),
                Some("name"),
                Some("asc"),
            )?;
            assert_eq!(users.len(), 1);
            Ok(())
        },
    )
}

#[test]
fn test_create_project_user() -> Result<()> {
    let create_user = CreateProjectUser {
        project_id: 123,
        user_id: 456,
        manager: Some(true),
        rate: Some(100.0),
        labour_cost: None,
    };

    let response = json!({
        "id": 1,
        "project_id": 123,
        "user_id": 456,
        "workspace_id": 1,
        "manager": true,
        "rate": 100.0,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1/project_users",
        200,
        Some(response),
        |client| {
            let user = client.projects().create_project_user(1, &create_user)?;
            assert_eq!(user.project_id, 123);
            assert!(user.manager);
            Ok(())
        },
    )
}

#[test]
fn test_bulk_edit_project_users() -> Result<()> {
    let operations = vec![PatchOperation {
        op: "replace".to_string(),
        path: "/manager".to_string(),
        value: Some(json!(false)),
    }];

    let response = json!([
        {
            "id": 1,
            "project_id": 123,
            "user_id": 456,
            "workspace_id": 1,
            "manager": false,
            "rate": 100.0,
            "at": "2023-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::PATCH,
        "/workspaces/1/project_users/1,2",
        200,
        Some(response),
        |client| {
            let users = client
                .projects()
                .bulk_edit_project_users(1, &[1, 2], &operations)?;
            assert_eq!(users.len(), 1);
            assert!(!users[0].manager);
            Ok(())
        },
    )
}

#[test]
fn test_update_project_user() -> Result<()> {
    let update = UpdateProjectUser {
        manager: Some(false),
        rate: Some(150.0),
        labour_cost: None,
    };

    let response = json!({
        "id": 1,
        "project_id": 123,
        "user_id": 456,
        "workspace_id": 1,
        "manager": false,
        "rate": 150.0,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1/project_users/1",
        200,
        Some(response),
        |client| {
            let user = client.projects().update_project_user(1, 1, &update)?;
            assert!(!user.manager);
            assert_eq!(user.rate, Some(150.0));
            Ok(())
        },
    )
}

#[test]
fn test_delete_project_user() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1/project_users/1",
        204,
        None,
        |client| {
            client.projects().delete_project_user(1, 1)?;
            Ok(())
        },
    )
}
