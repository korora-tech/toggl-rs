use crate::client::TogglClient;
use crate::models::api::workspace::*;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{ClientId, TagId, UserId, WorkspaceId};

use super::with_mockito;

#[test]
fn test_get_workspace() -> Result<()> {
    let response = json!({
        "id": 1234567,
        "organization_id": 7654321,
        "name": "My Workspace",
        "profile": 0,
        "premium": false,
        "business_ws": false,
        "admin": true,
        "default_hourly_rate": null,
        "rate_last_updated": null,
        "default_currency": "USD",
        "only_admins_may_create_projects": false,
        "only_admins_may_create_tags": false,
        "only_admins_see_billable_rates": false,
        "only_admins_see_team_dashboard": false,
        "projects_billable_by_default": true,
        "projects_enforce_billable": false,
        "projects_private_by_default": false,
        "reports_collapse": true,
        "rounding": 1,
        "rounding_minutes": 0,
        "api_token": null,
        "at": "2022-10-03T15:44:00.289146Z",
        "ical_enabled": false,
        "ical_url": null,
        "csv_upload": null,
        "subscription": null,
        "working_hours_in_minutes": null,
        "logo_url": null,
        "permissions": null,
        "max_data_retention_days": null
    });

    with_mockito(
        Method::GET,
        "/workspaces/1234567",
        200,
        Some(response),
        |client| {
            let workspace = client.workspaces().get(WorkspaceId(1234567))?;
            assert_eq!(WorkspaceId(1234567), workspace.id);
            assert_eq!("My Workspace", workspace.name);
            assert_eq!(true, workspace.admin);
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace() -> Result<()> {
    let update_data = UpdateWorkspace {
        name: Some("Updated Workspace".to_string()),
        only_admins_may_create_projects: Some(true),
        only_admins_may_create_tags: Some(true),
        only_admins_see_billable_rates: Some(true),
        only_admins_see_team_dashboard: Some(true),
        projects_billable_by_default: Some(false),
        projects_enforce_billable: Some(false),
        projects_private_by_default: Some(false),
        rate_change_mode: Some("start-today".to_string()),
        reports_collapse: Some(false),
        rounding: Some(1),
        rounding_minutes: Some(0),
        default_hourly_rate: None,
        default_currency: Some("USD".to_string()),
    };

    let response = json!({
        "id": 1234567,
        "organization_id": 7654321,
        "name": "Updated Workspace",
        "profile": 0,
        "premium": false,
        "business_ws": false,
        "admin": true,
        "default_hourly_rate": null,
        "rate_last_updated": null,
        "default_currency": "USD",
        "only_admins_may_create_projects": true,
        "only_admins_may_create_tags": true,
        "only_admins_see_billable_rates": true,
        "only_admins_see_team_dashboard": true,
        "projects_billable_by_default": false,
        "projects_enforce_billable": false,
        "projects_private_by_default": false,
        "reports_collapse": false,
        "rounding": 1,
        "rounding_minutes": 0,
        "api_token": null,
        "at": "2022-10-03T15:44:00.289146Z",
        "ical_enabled": false,
        "ical_url": null,
        "csv_upload": null,
        "subscription": null,
        "working_hours_in_minutes": null,
        "logo_url": null,
        "permissions": null,
        "max_data_retention_days": null
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567",
        200,
        Some(response),
        |client| {
            let workspace = client
                .workspaces()
                .update(WorkspaceId(1234567), &update_data)?;
            assert_eq!("Updated Workspace", workspace.name);
            assert_eq!(true, workspace.only_admins_may_create_projects);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_users() -> Result<()> {
    let response = json!([
        {
            "id": 1234567,
            "user_id": 123456,
            "workspace_id": 1234567,
            "name": "John Doe",
            "email": "john@example.com",
            "admin": true,
            "active": true,
            "invite_url": null,
            "invitation_code": null,
            "role": "admin",
            "at": "2022-10-03T15:44:00.289146Z",
            "is_direct": true,
            "labour_cost": null,
            "rate": null,
            "rate_last_updated": null,
            "group_ids": [],
            "avatar_file_name": null,
            "inactive": false,
            "timezone": "UTC",
            "working_hours_in_minutes": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/users",
        200,
        Some(response),
        |client| {
            let users = client.workspaces().get_users(WorkspaceId(1234567))?;
            assert_eq!(1, users.len());
            assert_eq!("John Doe", users[0].name);
            assert_eq!(true, users[0].admin);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_clients() -> Result<()> {
    let response = json!([
        {
            "id": 1234567,
            "workspace_id": 1234567,
            "name": "Important Client",
            "archived": false,
            "at": "2022-10-03T15:47:31+00:00",
            "server_deleted_at": null,
            "permissions": null
        },
        {
            "id": 7654321,
            "workspace_id": 1234567,
            "name": "Another Client",
            "archived": false,
            "at": "2022-10-03T21:21:29+00:00",
            "server_deleted_at": null,
            "permissions": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/clients",
        200,
        Some(response),
        |client| {
            let clients = client
                .workspaces()
                .get_clients(WorkspaceId(1234567), None)?;
            assert_eq!(2, clients.len());
            assert_eq!("Important Client", clients[0].name);
            assert_eq!("Another Client", clients[1].name);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_projects() -> Result<()> {
    let response = json!([
        {
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
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/projects",
        200,
        Some(response),
        |client| {
            let projects = client
                .workspaces()
                .get_projects(WorkspaceId(1234567), None, None)?;
            assert_eq!(1, projects.len());
            assert_eq!("Test Project", projects[0].name);
            Ok(())
        },
    )
}

#[test]
fn test_create_workspace() -> Result<()> {
    let create_data = CreateWorkspace {
        name: "New Workspace".to_string(),
        initial_pricing_plan: None,
    };

    let response = json!({
        "id": 9999999,
        "organization_id": null,
        "name": "New Workspace",
        "profile": 0,
        "premium": false,
        "business_ws": false,
        "admin": true,
        "default_hourly_rate": null,
        "rate_last_updated": null,
        "default_currency": "USD",
        "only_admins_may_create_projects": false,
        "only_admins_may_create_tags": false,
        "only_admins_see_billable_rates": false,
        "only_admins_see_team_dashboard": false,
        "projects_billable_by_default": true,
        "projects_enforce_billable": false,
        "projects_private_by_default": false,
        "reports_collapse": true,
        "rounding": 1,
        "rounding_minutes": 0,
        "api_token": null,
        "at": "2023-01-01T00:00:00Z",
        "ical_enabled": false,
        "ical_url": null,
        "csv_upload": null,
        "subscription": null,
        "working_hours_in_minutes": null,
        "logo_url": null,
        "permissions": null,
        "max_data_retention_days": null
    });

    with_mockito(Method::POST, "/workspaces", 200, Some(response), |client| {
        let workspace = client.workspaces().create(&create_data)?;
        assert_eq!(WorkspaceId(9999999), workspace.id);
        assert_eq!("New Workspace", workspace.name);
        Ok(())
    })
}

#[test]
fn test_get_workspace_alerts() -> Result<()> {
    let response = json!([{
        "id": 123,
        "workspace_id": 1234567,
        "alert_type": "budget_exceeded",
        "threshold": 1000.0,
        "enabled": true,
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": "2023-01-01T00:00:00Z",
        "meta": {
            "current_value": 1200.0,
            "triggered": true
        }
    }]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/alerts",
        200,
        Some(response),
        |client| {
            let alerts = client.workspaces().get_alerts(WorkspaceId(1234567))?;
            assert_eq!(1, alerts.len());
            assert_eq!("budget_exceeded", alerts[0].alert.alert_type);
            assert_eq!(true, alerts[0].meta.triggered);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_expenses() -> Result<()> {
    let response = json!([{
        "id": 456,
        "workspace_id": 1234567,
        "user_id": 123456,
        "project_id": null,
        "task_id": null,
        "spent_at": "2023-01-15",
        "description": "Flight to conference",
        "currency": "USD",
        "amount": 500.0,
        "category_id": 1,
        "category_name": "Travel",
        "billable": true,
        "payee": null,
        "receipt_url": null,
        "created_at": "2023-01-15T10:00:00Z",
        "updated_at": "2023-01-15T10:00:00Z",
        "at": "2023-01-15T10:00:00Z"
    }]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/expenses",
        200,
        Some(response),
        |client| {
            let expenses = client.workspaces().get_expenses(WorkspaceId(1234567))?;
            assert_eq!(1, expenses.len());
            assert_eq!("Flight to conference", expenses[0].description);
            assert_eq!(500.0, expenses[0].amount);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_currencies() -> Result<()> {
    let response = json!([
        {
            "code": "USD",
            "name": "United States Dollar",
            "symbol": "$"
        },
        {
            "code": "EUR",
            "name": "Euro",
            "symbol": "€"
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/1234567/currencies",
        200,
        Some(response),
        |client| {
            let currencies = client.workspaces().get_currencies(WorkspaceId(1234567))?;
            assert_eq!(2, currencies.len());
            assert_eq!("USD", currencies[0].code);
            assert_eq!("Euro", currencies[1].name);
            Ok(())
        },
    )
}

#[test]
fn test_add_workspace_user() -> Result<()> {
    let response = json!({
        "id": 999,
        "user_id": 888777,
        "workspace_id": 1234567,
        "name": "New User",
        "email": "newuser@example.com",
        "active": true,
        "admin": false,
        "owner": false,
        "api_token": null,
        "at": "2023-01-01T00:00:00Z",
        "role": "user",
        "rate": null,
        "rate_currency": null,
        "labour_cost": null,
        "default_currency": "USD",
        "group_ids": [],
        "avatar_file_name": null,
        "inactive": false,
        "is_direct": true,
        "timezone": "UTC",
        "working_hours_in_minutes": null,
        "invite_url": null,
        "rate_last_updated": null
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/workspace_users",
        200,
        Some(response),
        |client| {
            let user = client.workspaces().add_user(
                WorkspaceId(1234567),
                vec!["newuser@example.com".to_string()],
            )?;
            assert_eq!(user.user_id, UserId(888777));
            assert_eq!("New User", user.name);
            Ok(())
        },
    )
}

#[test]
fn test_create_tag() -> Result<()> {
    let response = json!({
        "id": 7777777,
        "workspace_id": 1234567,
        "name": "Important",
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/tags",
        200,
        Some(response),
        |client| {
            let tag = client
                .workspaces()
                .create_tag(WorkspaceId(1234567), "Important")?;
            assert_eq!(TagId(7777777), tag.id);
            assert_eq!("Important", tag.name);
            Ok(())
        },
    )
}

#[test]
fn test_update_tag() -> Result<()> {
    let response = json!({
        "id": 7777777,
        "workspace_id": 1234567,
        "name": "Very Important",
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567/tags/7777777",
        200,
        Some(response),
        |client| {
            let tag = client.workspaces().update_tag(
                WorkspaceId(1234567),
                TagId(7777777),
                "Very Important",
            )?;
            assert_eq!(TagId(7777777), tag.id);
            assert_eq!("Very Important", tag.name);
            Ok(())
        },
    )
}

#[test]
fn test_delete_tag() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/tags/7777777",
        200,
        None,
        |client| {
            client
                .workspaces()
                .delete_tag(WorkspaceId(1234567), TagId(7777777))?;
            Ok(())
        },
    )
}

#[test]
fn test_create_client() -> Result<()> {
    let response = json!({
        "id": 8888888,
        "workspace_id": 1234567,
        "name": "New Client",
        "archived": false,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/clients",
        200,
        Some(response),
        |client| {
            let new_client = client
                .workspaces()
                .create_client(WorkspaceId(1234567), "New Client")?;
            assert_eq!(new_client.id, ClientId(8888888));
            assert_eq!("New Client", new_client.name);
            Ok(())
        },
    )
}

#[test]
fn test_update_client() -> Result<()> {
    let response = json!({
        "id": 8888888,
        "workspace_id": 1234567,
        "name": "Updated Client",
        "archived": false,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567/clients/8888888",
        200,
        Some(response),
        |client| {
            let updated_client = client.workspaces().update_client(
                WorkspaceId(1234567),
                ClientId(8888888),
                "Updated Client",
            )?;
            assert_eq!(updated_client.id, ClientId(8888888));
            assert_eq!("Updated Client", updated_client.name);
            Ok(())
        },
    )
}

#[test]
fn test_delete_client() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/clients/8888888",
        200,
        None,
        |client| {
            client
                .workspaces()
                .delete_client(WorkspaceId(1234567), ClientId(8888888))?;
            Ok(())
        },
    )
}

#[test]
fn test_archive_client() -> Result<()> {
    let response = json!({
        "id": 8888888,
        "workspace_id": 1234567,
        "name": "Archived Client",
        "archived": true,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/clients/8888888/archive",
        200,
        Some(response),
        |client| {
            let archived_client = client
                .workspaces()
                .archive_client(WorkspaceId(1234567), ClientId(8888888))?;
            assert_eq!(archived_client.id, ClientId(8888888));
            assert_eq!(true, archived_client.archived);
            Ok(())
        },
    )
}

#[test]
fn test_restore_client() -> Result<()> {
    let response = json!({
        "id": 8888888,
        "workspace_id": 1234567,
        "name": "Restored Client",
        "archived": false,
        "at": "2023-01-01T00:00:00Z"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/clients/8888888/restore",
        200,
        Some(response),
        |client| {
            let restored_client = client
                .workspaces()
                .restore_client(WorkspaceId(1234567), ClientId(8888888))?;
            assert_eq!(restored_client.id, ClientId(8888888));
            assert_eq!(false, restored_client.archived);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_preferences() -> Result<()> {
    let response = json!({
        "logo": "https://example.com/logo.png"
    });

    with_mockito(
        Method::GET,
        "/workspaces/1234567/preferences",
        200,
        Some(response),
        |client| {
            let preferences = client.workspaces().get_preferences(WorkspaceId(1234567))?;
            assert_eq!(preferences.logo, "https://example.com/logo.png");
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace_preferences() -> Result<()> {
    use crate::models::api::preferences::WorkspacePreferences;

    let update_prefs = WorkspacePreferences {
        disable_approvals: Some(true),
        disable_expenses: Some(false),
        disable_timesheet_view: Some(false),
        hide_start_end_times: Some(true),
        inc_tos_accepted_at: None,
        inc_tos_accepted_by: None,
        initial_pricing_plan: None,
        report_locked_at: None,
        single_sign_on: Some(false),
        sso_requested_at: None,
    };

    let response = json!({
        "logo": "https://example.com/new-logo.png"
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/preferences",
        200,
        Some(response),
        |client| {
            let result = client
                .workspaces()
                .update_preferences(WorkspaceId(1234567), &update_prefs)?;
            assert_eq!(result.logo, "https://example.com/new-logo.png");
            Ok(())
        },
    )
}

#[test]
fn test_get_workspace_subscription() -> Result<()> {
    let response = json!({
        "active_users": 5,
        "auto_renew": true,
        "billing_period_in_months": 12,
        "campaign_available": false,
        "currency": "USD",
        "enterprise": false,
        "is_subscription_beta": false,
        "is_unified": true,
        "new_signup_trial": false,
        "payment_failed": false,
        "payment_method": "card",
        "plan_name": "Business",
        "seat_cost_in_cents": 1500,
        "seats": 10,
        "site": "toggl.com",
        "state": "active",
        "trial_available": false
    });

    with_mockito(
        Method::GET,
        "/workspaces/1234567/subscription",
        200,
        Some(response),
        |client| {
            let subscription = client.workspaces().get_subscription(WorkspaceId(1234567))?;
            assert_eq!(5, subscription.active_users);
            assert_eq!(true, subscription.auto_renew);
            assert_eq!("Business", subscription.plan_name);
            assert_eq!(1500, subscription.seat_cost_in_cents);
            assert_eq!(10, subscription.seats);
            Ok(())
        },
    )
}

#[test]
fn test_get_purchase_order_pdf() -> Result<()> {
    let pdf_content = vec![0x25, 0x50, 0x44, 0x46]; // PDF header bytes

    let mut server = mockito::Server::new();
    let mock = server
        .mock(
            "GET",
            "/workspaces/1234567/subscription/purchase_orders/789.pdf",
        )
        .with_status(200)
        .with_header("content-type", "application/pdf")
        .with_body(&pdf_content)
        .create();

    let toggl_client = TogglClient::new_with_base_url("test_api_token".to_string(), &server.url())?;
    let result = toggl_client
        .workspaces()
        .get_purchase_order_pdf(WorkspaceId(1234567), 789)?;

    // Verify we got PDF content back
    assert_eq!(result[0..4], pdf_content[..]);
    mock.assert();

    Ok(())
}
