use crate::models::api::group::{CreateGroup, UpdateGroup};
use crate::tests::*;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{GroupId, WorkspaceId};

#[test]
fn test_get_groups() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([
        {
            "id": 101,
            "workspace_id": 12345,
            "organization_id": 999,
            "name": "Developers",
            "at": "2024-01-01T10:00:00Z",
            "permissions": ["view_projects", "create_projects", "edit_projects"]
        },
        {
            "id": 102,
            "workspace_id": 12345,
            "organization_id": 999,
            "name": "Managers",
            "at": "2024-01-02T10:00:00Z",
            "permissions": ["view_projects", "create_projects", "edit_projects", "delete_projects"]
        }
    ]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/groups", workspace_id),
        200,
        Some(response),
        |client| {
            let groups = client.workspaces().get_groups(workspace_id)?;
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0].name, "Developers");
            assert_eq!(groups[0].id, GroupId(101));
            assert_eq!(groups[1].name, "Managers");
            assert_eq!(groups[1].permissions.as_ref().unwrap().len(), 4);
            Ok(())
        },
    )
}

#[test]
fn test_create_group() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let new_group = CreateGroup {
        workspace_id,
        name: "QA Team".to_string(),
    };

    let response = json!({
        "id": 103,
        "workspace_id": 12345,
        "organization_id": 999,
        "name": "QA Team",
        "at": "2024-01-17T15:00:00Z",
        "permissions": ["view_projects"]
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/groups", workspace_id),
        201,
        Some(response),
        |client| {
            let group = client.workspaces().create_group(workspace_id, &new_group)?;
            assert_eq!(group.id, GroupId(103));
            assert_eq!(group.name, "QA Team");
            assert_eq!(group.workspace_id, workspace_id);
            Ok(())
        },
    )
}

#[test]
fn test_get_group() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let group_id = GroupId(101);

    let response = json!({
        "id": 101,
        "workspace_id": 12345,
        "organization_id": 999,
        "name": "Developers",
        "at": "2024-01-01T10:00:00Z",
        "permissions": ["view_projects", "create_projects", "edit_projects"]
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/groups/{}", workspace_id, group_id),
        200,
        Some(response),
        |client| {
            let group = client.workspaces().get_group(workspace_id, group_id)?;
            assert_eq!(group.id, group_id);
            assert_eq!(group.name, "Developers");
            assert_eq!(group.permissions.as_ref().unwrap().len(), 3);
            Ok(())
        },
    )
}

#[test]
fn test_update_group() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let group_id = GroupId(101);

    let update = UpdateGroup {
        name: "Senior Developers".to_string(),
    };

    let response = json!({
        "id": 101,
        "workspace_id": 12345,
        "organization_id": 999,
        "name": "Senior Developers",
        "at": "2024-01-17T16:00:00Z",
        "permissions": ["view_projects", "create_projects", "edit_projects"]
    });

    with_mockito(
        Method::PUT,
        &format!("/workspaces/{}/groups/{}", workspace_id, group_id),
        200,
        Some(response),
        |client| {
            let group = client
                .workspaces()
                .update_group(workspace_id, group_id, &update)?;
            assert_eq!(group.id, group_id);
            assert_eq!(group.name, "Senior Developers");
            Ok(())
        },
    )
}

#[test]
fn test_delete_group() -> Result<()> {
    let workspace_id = WorkspaceId(12345);
    let group_id = GroupId(101);

    with_mockito(
        Method::DELETE,
        &format!("/workspaces/{}/groups/{}", workspace_id, group_id),
        204,
        None,
        |client| {
            client.workspaces().delete_group(workspace_id, group_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_groups() -> Result<()> {
    let workspace_id = WorkspaceId(12345);

    let response = json!([]);

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/groups", workspace_id),
        200,
        Some(response),
        |client| {
            let groups = client.workspaces().get_groups(workspace_id)?;
            assert!(groups.is_empty());
            Ok(())
        },
    )
}
