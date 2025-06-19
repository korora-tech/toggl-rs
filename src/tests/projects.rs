use crate::error::Result;
use crate::model::api::project::*;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::with_mockito;

#[test]
fn test_get_project() -> Result<()> {
    let response = json!({
        "id": 123456789,
        "workspace_id": 1234567,
        "client_id": null,
        "name": "Test Project",
        "is_private": true,
        "active": true,
        "at": "2023-01-02T23:02:17+00:00",
        "created_at": "2022-10-03T15:47:08+00:00",
        "server_deleted_at": null,
        "color": "#465bb3",
        "billable": false,
        "template": false,
        "auto_estimates": false,
        "estimated_hours": null,
        "estimated_seconds": null,
        "rate": null,
        "rate_last_updated": null,
        "currency": null,
        "recurring": false,
        "recurring_parameters": null,
        "current_period": null,
        "fixed_fee": null,
        "actual_hours": null,
        "actual_seconds": null,
        "start_date": null,
        "end_date": null,
        "permissions": null
    });

    with_mockito(
        Method::GET,
        "/workspaces/1234567/projects/123456789",
        200,
        Some(response),
        |client| {
            let project = client.projects().get(1234567, 123456789)?;
            assert_eq!(123456789, project.id);
            assert_eq!("Test Project", project.name);
            assert_eq!(true, project.active);
            Ok(())
        },
    )
}

#[test]
fn test_create_project() -> Result<()> {
    let create_data = CreateProject {
        workspace_id: 1234567,
        name: "New Project".to_string(),
        client_id: None,
        is_private: Some(true),
        active: Some(true),
        color: Some("#ff0000".to_string()),
        billable: Some(false),
        template: Some(false),
        auto_estimates: Some(false),
        estimated_hours: None,
        rate: None,
        currency: None,
        recurring: Some(false),
        recurring_parameters: None,
        fixed_fee: None,
        start_date: None,
        end_date: None,
    };

    let response = json!({
        "id": 999999999,
        "workspace_id": 1234567,
        "client_id": null,
        "name": "New Project",
        "is_private": true,
        "active": true,
        "at": "2023-01-01T00:00:00Z",
        "created_at": "2023-01-01T00:00:00Z",
        "server_deleted_at": null,
        "color": "#ff0000",
        "billable": false,
        "template": false,
        "auto_estimates": false,
        "estimated_hours": null,
        "estimated_seconds": null,
        "rate": null,
        "rate_last_updated": null,
        "currency": null,
        "recurring": false,
        "recurring_parameters": null,
        "current_period": null,
        "fixed_fee": null,
        "actual_hours": null,
        "actual_seconds": null,
        "start_date": null,
        "end_date": null,
        "permissions": null
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/projects",
        200,
        Some(response),
        |client| {
            let project = client.projects().create(1234567, &create_data)?;
            assert_eq!(999999999, project.id);
            assert_eq!("New Project", project.name);
            assert_eq!("#ff0000", project.color);
            Ok(())
        },
    )
}

#[test]
fn test_update_project() -> Result<()> {
    let update_data = UpdateProject {
        name: Some("Updated Project".to_string()),
        client_id: None,
        is_private: Some(false),
        active: Some(true),
        color: Some("#00ff00".to_string()),
        billable: Some(true),
        template: None,
        auto_estimates: None,
        estimated_hours: None,
        rate: None,
        currency: None,
        recurring: None,
        recurring_parameters: None,
        fixed_fee: None,
        start_date: None,
        end_date: None,
    };

    let response = json!({
        "id": 123456789,
        "workspace_id": 1234567,
        "client_id": null,
        "name": "Updated Project",
        "is_private": false,
        "active": true,
        "at": "2023-01-01T00:00:00Z",
        "created_at": "2022-10-03T15:47:08+00:00",
        "server_deleted_at": null,
        "color": "#00ff00",
        "billable": true,
        "template": false,
        "auto_estimates": false,
        "estimated_hours": null,
        "estimated_seconds": null,
        "rate": null,
        "rate_last_updated": null,
        "currency": null,
        "recurring": false,
        "recurring_parameters": null,
        "current_period": null,
        "fixed_fee": null,
        "actual_hours": null,
        "actual_seconds": null,
        "start_date": null,
        "end_date": null,
        "permissions": null
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567/projects/123456789",
        200,
        Some(response),
        |client| {
            let project = client.projects().update(1234567, 123456789, &update_data)?;
            assert_eq!("Updated Project", project.name);
            assert_eq!(false, project.is_private);
            assert_eq!(Some(true), project.billable);
            Ok(())
        },
    )
}

#[test]
fn test_delete_project() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/projects/123456789",
        200,
        None,
        |client| {
            client.projects().delete(1234567, 123456789)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_project_tasks() -> Result<()> {
    let response = json!([
        {
            "id": 111111,
            "name": "Design",
            "project_id": 123456789,
            "workspace_id": 1234567,
            "user_id": null,
            "recurring": false,
            "active": true,
            "at": "2023-01-01T00:00:00Z",
            "tracked_seconds": 0,
            "estimated_seconds": null
        },
        {
            "id": 222222,
            "name": "Development",
            "project_id": 123456789,
            "workspace_id": 1234567,
            "user_id": null,
            "recurring": false,
            "active": true,
            "at": "2023-01-01T00:00:00Z",
            "tracked_seconds": 0,
            "estimated_seconds": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/projects/123456789/tasks",
        200,
        Some(response),
        |client| {
            let tasks = client.projects().get_tasks(1234567, 123456789)?;
            assert_eq!(2, tasks.len());
            assert_eq!("Design", tasks[0].name);
            assert_eq!("Development", tasks[1].name);
            Ok(())
        },
    )
}

#[test]
fn test_create_project_task() -> Result<()> {
    let response = json!({
        "id": 333333,
        "name": "Testing",
        "project_id": 123456789,
        "workspace_id": 1234567,
        "user_id": null,
        "recurring": false,
        "active": true,
        "at": "2023-01-01T00:00:00Z",
        "tracked_seconds": 0,
        "estimated_seconds": null
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/projects/123456789/tasks",
        200,
        Some(response),
        |client| {
            let task = client
                .projects()
                .create_task(1234567, 123456789, "Testing", true)?;
            assert_eq!(333333, task.id);
            assert_eq!("Testing", task.name);
            assert_eq!(true, task.active);
            Ok(())
        },
    )
}

#[test]
fn test_update_project_task() -> Result<()> {
    let response = json!({
        "id": 333333,
        "name": "QA Testing",
        "project_id": 123456789,
        "workspace_id": 1234567,
        "user_id": null,
        "recurring": false,
        "active": false,
        "at": "2023-01-01T00:00:00Z",
        "tracked_seconds": 0,
        "estimated_seconds": null
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567/projects/123456789/tasks/333333",
        200,
        Some(response),
        |client| {
            let task =
                client
                    .projects()
                    .update_task(1234567, 123456789, 333333, "QA Testing", false)?;
            assert_eq!(333333, task.id);
            assert_eq!("QA Testing", task.name);
            assert_eq!(false, task.active);
            Ok(())
        },
    )
}

#[test]
fn test_delete_project_task() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/projects/123456789/tasks/333333",
        200,
        None,
        |client| {
            client.projects().delete_task(1234567, 123456789, 333333)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_project_statistics() -> Result<()> {
    let response = json!({
        "estimated_seconds": 36000,
        "tracked_seconds": 28800,
        "billable_seconds": 14400
    });

    with_mockito(
        Method::GET,
        "/workspaces/1234567/projects/123456789/statistics",
        200,
        Some(response),
        |client| {
            let stats = client.projects().get_statistics(1234567, 123456789)?;
            assert_eq!(Some(28800), stats.tracked_seconds);
            assert_eq!(Some(14400), stats.billable_seconds);
            Ok(())
        },
    )
}
