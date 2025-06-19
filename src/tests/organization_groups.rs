use crate::error::Result;
use crate::model::api::organization::{CreateOrganizationGroup, UpdateOrganizationGroup};
use crate::tests::*;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_organization_groups() -> Result<()> {
    let org_id = 12345;

    let response = json!([
        {
            "id": 101,
            "name": "Engineering Team",
            "workspace_id": 54321
        },
        {
            "id": 102,
            "name": "Marketing Team",
            "workspace_id": 54322
        },
        {
            "id": 103,
            "name": "Sales Team",
            "workspace_id": 54321
        }
    ]);

    with_mockito(
        Method::GET,
        &format!("/organizations/{}/groups", org_id),
        200,
        Some(response),
        |client| {
            let groups = client.organizations().get_groups(org_id)?;
            assert_eq!(groups.len(), 3);
            assert_eq!(groups[0].id, 101);
            assert_eq!(groups[0].name, "Engineering Team");
            assert_eq!(groups[0].workspace_id, 54321);
            assert_eq!(groups[1].name, "Marketing Team");
            assert_eq!(groups[2].name, "Sales Team");
            Ok(())
        },
    )
}

#[test]
fn test_create_organization_group() -> Result<()> {
    let org_id = 12345;

    let new_group = CreateOrganizationGroup {
        name: "Design Team".to_string(),
        workspace_id: 54321,
    };

    let response = json!({
        "id": 104,
        "name": "Design Team",
        "workspace_id": 54321
    });

    with_mockito(
        Method::POST,
        &format!("/organizations/{}/groups", org_id),
        201,
        Some(response),
        |client| {
            let group = client.organizations().create_group(org_id, &new_group)?;
            assert_eq!(group.id, 104);
            assert_eq!(group.name, "Design Team");
            assert_eq!(group.workspace_id, 54321);
            Ok(())
        },
    )
}

#[test]
fn test_update_organization_group() -> Result<()> {
    let org_id = 12345;
    let group_id = 101;

    let update_group = UpdateOrganizationGroup {
        name: "Updated Engineering Team".to_string(),
    };

    let response = json!({
        "id": 101,
        "name": "Updated Engineering Team",
        "workspace_id": 54321
    });

    with_mockito(
        Method::PUT,
        &format!("/organizations/{}/groups/{}", org_id, group_id),
        200,
        Some(response),
        |client| {
            let group = client
                .organizations()
                .update_group(org_id, group_id, &update_group)?;
            assert_eq!(group.id, 101);
            assert_eq!(group.name, "Updated Engineering Team");
            assert_eq!(group.workspace_id, 54321);
            Ok(())
        },
    )
}

#[test]
fn test_patch_organization_group() -> Result<()> {
    let org_id = 12345;
    let group_id = 102;

    let patch_group = UpdateOrganizationGroup {
        name: "Patched Marketing Team".to_string(),
    };

    let response = json!({
        "id": 102,
        "name": "Patched Marketing Team",
        "workspace_id": 54322
    });

    with_mockito(
        Method::PATCH,
        &format!("/organizations/{}/groups/{}", org_id, group_id),
        200,
        Some(response),
        |client| {
            let group = client
                .organizations()
                .patch_group(org_id, group_id, &patch_group)?;
            assert_eq!(group.id, 102);
            assert_eq!(group.name, "Patched Marketing Team");
            assert_eq!(group.workspace_id, 54322);
            Ok(())
        },
    )
}

#[test]
fn test_delete_organization_group() -> Result<()> {
    let org_id = 12345;
    let group_id = 103;

    with_mockito(
        Method::DELETE,
        &format!("/organizations/{}/groups/{}", org_id, group_id),
        204,
        None,
        |client| {
            client.organizations().delete_group(org_id, group_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_groups_in_organization() -> Result<()> {
    let org_id = 12345;
    let workspace_id = 54321;

    let response = json!([
        {
            "id": 201,
            "name": "Workspace Group 1",
            "workspace_id": 54321
        },
        {
            "id": 202,
            "name": "Workspace Group 2",
            "workspace_id": 54321
        }
    ]);

    with_mockito(
        Method::GET,
        &format!(
            "/organizations/{}/workspaces/{}/groups",
            org_id, workspace_id
        ),
        200,
        Some(response),
        |client| {
            let groups = client
                .organizations()
                .get_workspace_groups(org_id, workspace_id)?;
            assert_eq!(groups.len(), 2);
            assert_eq!(groups[0].id, 201);
            assert_eq!(groups[0].name, "Workspace Group 1");
            assert_eq!(groups[1].id, 202);
            assert_eq!(groups[1].name, "Workspace Group 2");
            Ok(())
        },
    )
}

#[test]
fn test_get_empty_organization_groups() -> Result<()> {
    let org_id = 12345;

    let response = json!([]);

    with_mockito(
        Method::GET,
        &format!("/organizations/{}/groups", org_id),
        200,
        Some(response),
        |client| {
            let groups = client.organizations().get_groups(org_id)?;
            assert!(groups.is_empty());
            Ok(())
        },
    )
}
