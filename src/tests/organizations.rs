use crate::error::Result;
use crate::model::api::organization::*;
use crate::model::api::workspace::CreateWorkspace;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;

use super::with_mockito;

#[test]
fn test_get_organization() -> Result<()> {
    let response = json!({
        "id": 1234567,
        "name": "Test Organization",
        "pricing_plan_id": 456,
        "created_at": "2023-01-01T00:00:00Z",
        "at": "2023-01-01T00:00:00Z",
        "server_deleted_at": null,
        "is_unified": false,
        "max_workspaces": 20,
        "admin": true,
        "owner": true,
        "suspended_at": null,
        "is_multi_workspace_enabled": true,
        "is_chargify": false,
        "user_count": 5,
        "trial_info": {
            "trial": false,
            "trial_available": true,
            "trial_end_date": null,
            "next_payment_date": null,
            "last_pricing_plan_id": null
        },
        "payment_methods": null,
        "permissions": null
    });

    with_mockito(
        Method::GET,
        "/organizations/1234567",
        200,
        Some(response),
        |client| {
            let org = client.organizations().get(1234567)?;
            assert_eq!(1234567, org.id);
            assert_eq!("Test Organization", org.name);
            assert_eq!(true, org.admin);
            Ok(())
        },
    )
}

#[test]
fn test_get_organization_invitations() -> Result<()> {
    let response = json!([{
        "id": 12345,
        "email": "invited@example.com",
        "invitation_id": "INV123",
        "invite_url": "https://toggl.com/invite/ABC123",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z",
        "deleted_at": null,
        "sender_id": 98765,
        "sender_name": "Admin User",
        "sender_email": "admin@example.com",
        "recipient_id": null,
        "recipient_email": "invited@example.com",
        "recipient_name": null,
        "workspace_id": 777888,
        "workspace_name": "Main Workspace",
        "organization_id": 1234567,
        "organization_name": "Test Organization"
    }]);

    with_mockito(
        Method::GET,
        "/organizations/1234567/invitations",
        200,
        Some(response),
        |client| {
            let invitations = client.organizations().get_invitations(1234567)?;
            assert_eq!(1, invitations.len());
            assert_eq!("invited@example.com", invitations[0].email);
            Ok(())
        },
    )
}

#[test]
fn test_accept_organization_invitation() -> Result<()> {
    with_mockito(
        Method::POST,
        "/organizations/invitations/ABC123/accept",
        200,
        None,
        |client| {
            client.organizations().accept_invitation("ABC123")?;
            Ok(())
        },
    )
}

#[test]
fn test_reject_organization_invitation() -> Result<()> {
    with_mockito(
        Method::POST,
        "/organizations/invitations/ABC123/reject",
        200,
        None,
        |client| {
            client.organizations().reject_invitation("ABC123")?;
            Ok(())
        },
    )
}

#[test]
fn test_leave_organization() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/organizations/1234567/users/leave",
        200,
        None,
        |client| {
            client.organizations().leave(1234567)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_detailed_organization_users() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "user_id": 456,
            "organization_id": 123,
            "name": "John Doe",
            "email": "john@example.com",
            "admin": true,
            "owner": false,
            "active": true,
            "joined": true,
            "labour_cost": 50.0,
            "role": "developer",
            "created_at": "2023-01-01T00:00:00Z",
            "updated_at": "2023-01-01T00:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/users/detailed",
        200,
        Some(response),
        |client| {
            let users = client
                .organizations()
                .get_users_detailed(123, None, None, None, None, None)?;
            assert_eq!(users.len(), 1);
            assert_eq!(users[0].labour_cost, Some(50.0));
            assert_eq!(users[0].role, Some("developer".to_string()));
            Ok(())
        },
    )
}

#[test]
fn test_update_organization_user() -> Result<()> {
    let update = UpdateOrganizationUser {
        admin: Some(true),
        owner: None,
        active: Some(true),
    };

    let response = json!({
        "id": 1,
        "user_id": 456,
        "organization_id": 123,
        "name": "John Doe",
        "email": "john@example.com",
        "admin": true,
        "owner": false,
        "active": true,
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::PUT,
        "/organizations/123/users/456",
        200,
        Some(response),
        |client| {
            let user = client.organizations().update_user(123, 456, &update)?;
            assert_eq!(user.admin, true);
            assert_eq!(user.active, true);
            Ok(())
        },
    )
}

#[test]
fn test_get_organization_roles() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "name": "Admin",
            "permissions": ["manage_workspace", "manage_users", "view_reports"]
        },
        {
            "id": 2,
            "name": "Member",
            "permissions": ["track_time", "view_reports"]
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/roles",
        200,
        Some(response),
        |client| {
            let roles = client.organizations().get_roles(123)?;
            assert_eq!(roles.len(), 2);
            assert_eq!(roles[0].name, "Admin");
            assert_eq!(roles[0].permissions.len(), 3);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_statistics() -> Result<()> {
    let response = json!([
        {
            "workspace_id": 456,
            "billable_seconds": 28800,
            "tracked_seconds": 36000,
            "active_member_count": 10,
            "inactive_member_count": 2
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/workspaces/statistics",
        200,
        Some(response),
        |client| {
            let stats = client.organizations().get_workspace_statistics(123)?;
            assert_eq!(stats.len(), 1);
            assert_eq!(stats[0].workspace_id, 456);
            assert_eq!(stats[0].billable_seconds, 28800);
            assert_eq!(stats[0].active_member_count, 10);
            Ok(())
        },
    )
}

#[test]
fn test_create_organization_workspace() -> Result<()> {
    let workspace = CreateWorkspace {
        name: "New Workspace".to_string(),
        initial_pricing_plan: Some(1),
    };

    let response = json!({
        "id": 789,
        "name": "New Workspace",
        "profile": 0,
        "premium": false,
        "business_ws": false,
        "admin": true,
        "default_currency": "USD",
        "only_admins_may_create_projects": false,
        "only_admins_may_create_tags": false,
        "only_admins_see_billable_rates": false,
        "only_admins_see_team_dashboard": false,
        "projects_billable_by_default": true,
        "projects_enforce_billable": false,
        "projects_private_by_default": false,
        "reports_collapse": false,
        "rounding": 0,
        "rounding_minutes": 0,
        "ical_enabled": false,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/organizations/123/workspaces",
        201,
        Some(response),
        |client| {
            let created = client.organizations().create_workspace(123, &workspace)?;
            assert_eq!(created.id, 789);
            assert_eq!(created.name, "New Workspace");
            Ok(())
        },
    )
}

#[test]
fn test_ownership_transfer_flow() -> Result<()> {
    // Test getting transfers
    let transfers_response = json!([
        {
            "id": 1,
            "organization_id": 123,
            "current_owner_id": 456,
            "new_owner_id": 789,
            "status": "pending",
            "created_at": "2024-01-15T10:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/123/owner/transfer",
        200,
        Some(transfers_response),
        |client| {
            let transfers = client.organizations().get_ownership_transfers(123)?;
            assert_eq!(transfers.len(), 1);
            assert_eq!(transfers[0].status, "pending");
            Ok(())
        },
    )?;

    // Test creating transfer
    let create_transfer = CreateOwnershipTransfer { new_owner_id: 789 };

    let create_response = json!({
        "id": 2,
        "organization_id": 123,
        "current_owner_id": 456,
        "new_owner_id": 789,
        "status": "pending",
        "created_at": "2024-01-16T10:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/organizations/123/owner/transfer",
        201,
        Some(create_response),
        |client| {
            let transfer = client
                .organizations()
                .create_ownership_transfer(123, &create_transfer)?;
            assert_eq!(transfer.id, 2);
            assert_eq!(transfer.new_owner_id, 789);
            Ok(())
        },
    )?;

    // Test accepting transfer
    with_mockito(
        Method::POST,
        "/organizations/123/owner/transfer/2/accept",
        200,
        None,
        |client| {
            client
                .organizations()
                .handle_ownership_transfer(123, 2, "accept")?;
            Ok(())
        },
    )
}

#[test]
fn test_get_organization_owner() -> Result<()> {
    let response = json!({
        "id": 98765,
        "email": "owner@example.com",
        "name": "Owner Name",
        "created_at": "2023-01-01T10:00:00Z"
    });

    with_mockito(
        Method::GET,
        "/organizations/12345/owner",
        200,
        Some(response),
        |client| {
            let owner = client.organizations().get_owner(12345)?;
            assert_eq!(owner.id, 98765);
            assert_eq!(owner.email, "owner@example.com");
            assert_eq!(owner.name, "Owner Name");
            Ok(())
        },
    )
}

#[test]
fn test_get_organization_users() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "user_id": 98765,
            "organization_id": 12345,
            "name": "John Doe",
            "email": "john@example.com",
            "admin": true,
            "owner": false,
            "active": true,
            "avatar_file_name": "avatar1.jpg",
            "created_at": "2023-01-01T10:00:00Z",
            "updated_at": "2023-01-01T10:00:00Z"
        },
        {
            "id": 2,
            "user_id": 98766,
            "organization_id": 12345,
            "name": "Jane Smith",
            "email": "jane@example.com",
            "admin": false,
            "owner": false,
            "active": true,
            "avatar_file_name": null,
            "created_at": "2023-01-02T10:00:00Z",
            "updated_at": "2023-01-02T10:00:00Z"
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/12345/users",
        200,
        Some(response),
        |client| {
            let users = client.organizations().get_users(12345, None, None)?;
            assert_eq!(users.len(), 2);
            assert_eq!(users[0].name, "John Doe");
            assert!(users[0].admin);
            assert_eq!(users[1].name, "Jane Smith");
            assert!(!users[1].admin);
            Ok(())
        },
    )
}

#[test]
fn test_get_organization_workspaces() -> Result<()> {
    let response = json!([
        {
            "id": 54321,
            "name": "Main Workspace",
            "profile": 0,
            "premium": true,
            "admin": true,
            "default_hourly_rate": 50.0,
            "default_currency": "USD",
            "only_admins_may_create_projects": false,
            "only_admins_see_billable_rates": true,
            "only_admins_see_team_dashboard": false,
            "projects_billable_by_default": true,
            "rate_change_mode": "start-today",
            "rounding": 1,
            "rounding_minutes": 1,
            "at": "2024-01-01T10:00:00Z",
            "logo_url": "https://example.com/logo.png",
            "permissions": null
        },
        {
            "id": 54322,
            "name": "Secondary Workspace",
            "profile": 0,
            "premium": false,
            "admin": false,
            "default_hourly_rate": null,
            "default_currency": "EUR",
            "only_admins_may_create_projects": true,
            "only_admins_see_billable_rates": true,
            "only_admins_see_team_dashboard": true,
            "projects_billable_by_default": false,
            "rate_change_mode": "start-today",
            "rounding": 1,
            "rounding_minutes": 1,
            "at": "2024-01-02T10:00:00Z",
            "logo_url": null,
            "permissions": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/organizations/12345/workspaces",
        200,
        Some(response),
        |client| {
            let workspaces = client.organizations().get_workspaces(12345)?;
            assert_eq!(workspaces.len(), 2);
            assert_eq!(workspaces[0].name, "Main Workspace");
            assert!(workspaces[0].admin);
            assert_eq!(workspaces[1].name, "Secondary Workspace");
            assert!(!workspaces[1].admin);
            Ok(())
        },
    )
}

#[test]
fn test_update_organization() -> Result<()> {
    let update = UpdateOrganization {
        name: Some("Updated Organization Name".to_string()),
    };

    let response = json!({
        "id": 12345,
        "name": "Updated Organization Name",
        "pricing_plan_id": 1,
        "created_at": "2023-01-01T10:00:00Z",
        "at": "2024-01-16T15:00:00Z",
        "server_deleted_at": null,
        "is_unified": true,
        "is_multi_workspace_enabled": true,
        "is_chargify": false,
        "max_workspaces": 10,
        "admin": true,
        "owner": true,
        "suspended_at": null,
        "user_count": 25,
        "trial_info": {
            "trial": false,
            "trial_available": true,
            "trial_end_date": null,
            "next_payment_date": "2024-02-01T00:00:00Z",
            "last_pricing_plan_id": null
        },
        "payment_methods": "card",
        "permissions": ["can_create_workspaces", "can_edit_organization"]
    });

    with_mockito(
        Method::PUT,
        "/organizations/12345",
        200,
        Some(response),
        |client| {
            let org = client.organizations().update(12345, &update)?;
            assert_eq!(org.name, "Updated Organization Name");
            assert!(org.is_multi_workspace_enabled);
            Ok(())
        },
    )
}

#[test]
fn test_resend_organization_invitation() -> Result<()> {
    with_mockito(
        Method::PUT,
        "/organizations/12345/invitations/67890/resend",
        200,
        None,
        |client| {
            client.organizations().resend_invitation(12345, 67890)?;
            Ok(())
        },
    )
}
