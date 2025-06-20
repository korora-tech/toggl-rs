use crate::models::api::user::{ResetToken, UpdateUser};
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{
    ClientId, OrganizationId, ProjectId, ReminderId, TagId, TaskId, TimeEntryId, UserId,
    WorkspaceId,
};

use super::{with_mockito, with_mockito_params, API_TOKEN};

#[test]
fn test_get_me() -> Result<()> {
    let response = json!({
        "id": 1234567,
        "api_token": API_TOKEN,
        "email": "test@example.com",
        "fullname": "Test User",
        "timezone": "Europe/Berlin",
        "default_workspace_id": 9876543,
        "beginning_of_week": 1,
        "image_url": "https://assets.track.toggl.com/images/profile.png",
        "created_at": "2020-01-01T00:00:00+00:00",
        "updated_at": "2020-01-01T00:00:00+00:00",
        "openid_email": null,
        "openid_enabled": false,
        "country_id": 82,
        "at": "2020-01-01T00:00:00+00:00",
        "intercom_hash": "hash123",
        "has_password": true,
        "options": null,
        "tags": null
    });

    with_mockito(Method::GET, "/me", 200, Some(response), |client| {
        let user = client.me().get()?;
        assert_eq!(user.id, UserId(1234567));
        assert_eq!("test@example.com", user.email);
        assert_eq!("Test User", user.fullname);
        assert_eq!(true, user.has_password);
        Ok(())
    })
}

#[test]
fn test_update_me() -> Result<()> {
    let update_data = UpdateUser {
        fullname: Some("Updated Name".to_string()),
        email: None,
        timezone: None,
        default_workspace_id: None,
        beginning_of_week: None,
        country_id: None,
        password: None,
        current_password: None,
    };

    let response = json!({
        "id": 1234567,
        "api_token": API_TOKEN,
        "email": "test@example.com",
        "fullname": "Updated Name",
        "timezone": "Europe/Berlin",
        "default_workspace_id": 9876543,
        "beginning_of_week": 1,
        "image_url": "https://assets.track.toggl.com/images/profile.png",
        "created_at": "2020-01-01T00:00:00+00:00",
        "updated_at": "2020-01-01T00:00:00+00:00",
        "openid_email": null,
        "openid_enabled": false,
        "country_id": 82,
        "at": "2020-01-01T00:00:00+00:00",
        "intercom_hash": "hash123",
        "has_password": true,
        "options": null,
        "tags": null
    });

    with_mockito(Method::PUT, "/me", 200, Some(response), |client| {
        let user = client.me().update(&update_data)?;
        assert_eq!("Updated Name", user.fullname);
        Ok(())
    })
}

#[test]
fn test_get_clients() -> Result<()> {
    let response = json!([
        {
            "id": 1234567,
            "workspace_id": 9876543,
            "name": "Client One",
            "archived": false,
            "at": "2022-10-03T15:47:31+00:00",
            "server_deleted_at": null,
            "permissions": null
        },
        {
            "id": 7654321,
            "workspace_id": 9876543,
            "name": "Client Two",
            "archived": false,
            "at": "2022-10-03T21:21:29+00:00",
            "server_deleted_at": null,
            "permissions": null
        }
    ]);

    with_mockito(Method::GET, "/me/clients", 200, Some(response), |client| {
        let clients = client.me().get_clients(None)?;
        assert_eq!(2, clients.len());
        assert_eq!(clients[0].id, ClientId(1234567));
        assert_eq!("Client One", clients[0].name);
        assert_eq!(clients[1].id, ClientId(7654321));
        assert_eq!("Client Two", clients[1].name);
        Ok(())
    })
}

#[test]
fn test_get_features() -> Result<()> {
    let response = json!([{
        "workspace_id": 1234567,
        "features": [
            {
                "feature_id": 0,
                "name": "free",
                "enabled": true
            },
            {
                "feature_id": 13,
                "name": "pro",
                "enabled": false
            },
            {
                "feature_id": 15,
                "name": "business",
                "enabled": false
            }
        ]
    }]);

    with_mockito(Method::GET, "/me/features", 200, Some(response), |client| {
        let features = client.me().get_features()?;
        assert_eq!(1, features.len());
        assert_eq!(features[0].workspace_id, WorkspaceId(1234567));
        assert_eq!(3, features[0].features.len());
        assert_eq!("free", features[0].features[0].name);
        assert_eq!(true, features[0].features[0].enabled);
        Ok(())
    })
}

#[test]
fn test_get_location() -> Result<()> {
    let response = json!({
        "city": "Berlin",
        "city_lat_long": "52.520008,13.404954",
        "state": "Berlin",
        "country_code": "DE",
        "country_name": "Germany"
    });

    with_mockito(Method::GET, "/me/location", 200, Some(response), |client| {
        let location = client.me().get_location()?;
        assert_eq!("Berlin", location.city);
        assert_eq!("52.520008,13.404954", location.city_lat_long);
        assert_eq!("DE", location.country_code);
        Ok(())
    })
}

#[test]
fn test_get_logged() -> Result<()> {
    with_mockito(Method::GET, "/me/logged", 200, None, |client| {
        client.me().get_logged()?;
        Ok(())
    })
}

#[test]
fn test_get_organizations() -> Result<()> {
    let response = json!([
        {
            "id": 1234567,
            "name": "My Organization",
            "pricing_plan_id": 0,
            "created_at": "2021-11-12T19:33:19.860863Z",
            "at": "2021-11-12T19:33:19.930603Z",
            "server_deleted_at": null,
            "is_multi_workspace_enabled": false,
            "suspended_at": null,
            "user_count": 5,
            "trial_info": {
                "trial": false,
                "trial_available": false,
                "trial_end_date": null,
                "next_payment_date": null,
                "last_pricing_plan_id": null
            },
            "is_chargify": false,
            "is_unified": false,
            "max_workspaces": 20,
            "admin": true,
            "owner": true,
            "permissions": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/me/organizations",
        200,
        Some(response),
        |client| {
            let orgs = client.me().get_organizations()?;
            assert_eq!(1, orgs.len());
            assert_eq!(OrganizationId(1234567), orgs[0].id);
            assert_eq!("My Organization", orgs[0].name);
            assert_eq!(true, orgs[0].admin);
            assert_eq!(true, orgs[0].owner);
            Ok(())
        },
    )
}

#[test]
fn test_get_projects() -> Result<()> {
    let response = json!([
        {
            "id": 123456789,
            "workspace_id": 123456789,
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

    with_mockito(Method::GET, "/me/projects", 200, Some(response), |client| {
        let projects = client.me().get_projects(None, None)?;
        assert_eq!(1, projects.len());
        assert_eq!(ProjectId(123456789), projects[0].id);
        assert_eq!("Test Project", projects[0].name);
        assert_eq!(true, projects[0].active);
        Ok(())
    })
}

#[test]
fn test_get_tags() -> Result<()> {
    let response = json!([
        {
            "id": 1234,
            "workspace_id": 123456789,
            "name": "important",
            "at": "2022-10-03T15:44:20.424008Z",
            "deleted_at": null,
            "permissions": null
        },
        {
            "id": 1235,
            "workspace_id": 123456789,
            "name": "urgent",
            "at": "2022-10-03T15:49:09.73311Z",
            "deleted_at": null,
            "permissions": null
        }
    ]);

    with_mockito(Method::GET, "/me/tags", 200, Some(response), |client| {
        let tags = client.me().get_tags(None)?;
        assert_eq!(2, tags.len());
        assert_eq!(tags[0].id, TagId(1234));
        assert_eq!("important", tags[0].name);
        assert_eq!(tags[1].id, TagId(1235));
        assert_eq!("urgent", tags[1].name);
        Ok(())
    })
}

#[test]
fn test_get_tasks() -> Result<()> {
    let response = json!([
        {
            "id": 1234,
            "name": "Task One",
            "workspace_id": 123456789,
            "project_id": 123456789,
            "user_id": null,
            "recurring": false,
            "active": true,
            "at": "2023-01-08T00:03:11+00:00",
            "server_deleted_at": null,
            "estimated_seconds": 0,
            "tracked_seconds": 0,
            "permissions": null
        },
        {
            "id": 1235,
            "name": "Task Two",
            "workspace_id": 123456789,
            "project_id": 123456789,
            "user_id": null,
            "recurring": false,
            "active": false,
            "at": "2023-01-08T00:03:11+00:00",
            "server_deleted_at": null,
            "estimated_seconds": 0,
            "tracked_seconds": 0,
            "permissions": null
        }
    ]);

    with_mockito(Method::GET, "/me/tasks", 200, Some(response), |client| {
        let tasks = client.me().get_tasks(None, None)?;
        assert_eq!(2, tasks.len());
        assert_eq!(TaskId(1234), tasks[0].id);
        assert_eq!("Task One", tasks[0].name);
        assert_eq!(true, tasks[0].active);
        assert_eq!(false, tasks[1].active);
        Ok(())
    })
}

#[test]
fn test_get_track_reminders() -> Result<()> {
    let response = json!([
        {
            "reminder_id": 5490,
            "workspace_id": 6967122,
            "frequency": 1,
            "threshold": 2,
            "created_at": "2023-01-08T00:10:32.840314Z",
            "user_ids": null,
            "group_ids": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/me/track_reminders",
        200,
        Some(response),
        |client| {
            let reminders = client.me().get_track_reminders()?;
            assert_eq!(1, reminders.len());
            assert_eq!(reminders[0].reminder_id, ReminderId(5490));
            assert_eq!(1, reminders[0].frequency);
            assert_eq!(2, reminders[0].threshold);
            Ok(())
        },
    )
}

#[test]
fn test_get_workspaces() -> Result<()> {
    let response = json!([
        {
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
        }
    ]);

    with_mockito(
        Method::GET,
        "/me/workspaces",
        200,
        Some(response),
        |client| {
            let workspaces = client.me().get_workspaces(None)?;
            assert_eq!(1, workspaces.len());
            assert_eq!(WorkspaceId(1234567), workspaces[0].id);
            assert_eq!("My Workspace", workspaces[0].name);
            assert_eq!(true, workspaces[0].admin);
            Ok(())
        },
    )
}

#[test]
fn test_accept_tos() -> Result<()> {
    with_mockito(Method::GET, "/me/accept_tos", 200, None, |client| {
        client.me().accept_tos()?;
        Ok(())
    })
}

#[test]
fn test_close_account() -> Result<()> {
    with_mockito(Method::POST, "/me/close_account", 200, None, |client| {
        client.me().close_account()?;
        Ok(())
    })
}

#[test]
fn test_disable_product_emails() -> Result<()> {
    with_mockito(
        Method::POST,
        "/me/disable_product_emails/test123",
        200,
        None,
        |client| {
            client.me().disable_product_emails("test123")?;
            Ok(())
        },
    )
}

#[test]
fn test_disable_weekly_report() -> Result<()> {
    with_mockito(
        Method::POST,
        "/me/disable_weekly_report/test456",
        200,
        None,
        |client| {
            client.me().disable_weekly_report("test456")?;
            Ok(())
        },
    )
}

#[test]
fn test_get_flags() -> Result<()> {
    let response = json!({
        "feature_a": true,
        "feature_b": false,
        "feature_c": "value",
        "feature_d": 123
    });

    with_mockito(Method::GET, "/me/flags", 200, Some(response), |client| {
        let flags = client.me().get_flags()?;
        assert_eq!(4, flags.0.len());
        assert_eq!(Some(&json!(true)), flags.0.get("feature_a"));
        assert_eq!(Some(&json!(false)), flags.0.get("feature_b"));
        Ok(())
    })
}

#[test]
fn test_get_preferences() -> Result<()> {
    let response = json!({
        "date_format": "MM/DD/YYYY",
        "duration_format": "improved",
        "timeofday_format": "h:mm A",
        "beginning_of_week": 1,
        "theme": "dark"
    });

    with_mockito(
        Method::GET,
        "/me/preferences",
        200,
        Some(response),
        |client| {
            let prefs = client.me().get_preferences()?;
            assert_eq!(Some("MM/DD/YYYY".to_string()), prefs.date_format);
            assert_eq!(Some("improved".to_string()), prefs.duration_format);
            assert_eq!(Some("dark".to_string()), prefs.theme);
            Ok(())
        },
    )
}

#[test]
fn test_get_push_services() -> Result<()> {
    let response = json!([{
        "id": 12345,
        "token": "test-token-123",
        "platform": "ios",
        "device_name": "iPhone 12"
    }]);

    with_mockito(
        Method::GET,
        "/me/push_services",
        200,
        Some(response),
        |client| {
            let services = client.me().get_push_services()?;
            assert_eq!(1, services.len());
            assert_eq!(12345, services[0].id);
            assert_eq!("ios", services[0].platform);
            Ok(())
        },
    )
}

#[test]
fn test_get_quota() -> Result<()> {
    let response = json!({
        "total": 1000000,
        "used": 250000,
        "available": 750000
    });

    with_mockito(Method::GET, "/me/quota", 200, Some(response), |client| {
        let quota = client.me().get_quota()?;
        assert_eq!(1000000, quota.total);
        assert_eq!(250000, quota.used);
        assert_eq!(750000, quota.available);
        Ok(())
    })
}

#[test]
fn test_get_shared_time_entries() -> Result<()> {
    let response = json!([{
        "time_entry_id": 987654321,
        "user_id": 123456,
        "user_name": "John Doe",
        "user_email": "john@example.com",
        "shared_at": "2023-01-01T10:00:00Z"
    }]);

    with_mockito(
        Method::GET,
        "/me/time_entries_shared_with",
        200,
        Some(response),
        |client| {
            let shared = client.me().get_shared_time_entries()?;
            assert_eq!(1, shared.len());
            assert_eq!(shared[0].time_entry_id, TimeEntryId(987654321));
            assert_eq!("John Doe", shared[0].user_name);
            Ok(())
        },
    )
}

#[test]
fn test_get_web_timer() -> Result<()> {
    let response = json!({
        "web_timer_id": "timer-12345",
        "web_timer_seconds": 3600
    });

    with_mockito(
        Method::GET,
        "/me/web-timer",
        200,
        Some(response),
        |client| {
            let web_timer = client.me().get_web_timer()?;
            assert_eq!(web_timer.web_timer_id, "timer-12345");
            assert_eq!(web_timer.web_timer_seconds, 3600);
            Ok(())
        },
    )
}

#[test]
fn test_request_export() -> Result<()> {
    let response = json!("export-uuid-12345");

    let mut params = BTreeMap::new();
    params.insert("export_type", "csv");

    with_mockito_params(
        Method::GET,
        "/me/export",
        Some(params),
        200,
        Some(response),
        |client| {
            let export_uuid = client.me().request_export("csv")?;
            assert_eq!(export_uuid, "export-uuid-12345");
            Ok(())
        },
    )
}

#[test]
fn test_reset_token() -> Result<()> {
    let reset_request = ResetToken {
        token: "current-token".to_string(),
        password: "password123".to_string(),
        email: "user@example.com".to_string(),
    };

    let response = json!({
        "id": 98765,
        "email": "user@example.com",
        "fullname": "Test User",
        "api_token": "new-token-12345",
        "timezone": "America/New_York",
        "default_workspace_id": 54321,
        "beginning_of_week": 1,
        "image_url": "https://example.com/avatar.jpg",
        "created_at": "2023-01-01T10:00:00Z",
        "updated_at": "2024-01-16T15:00:00Z",
        "at": "2024-01-16T15:00:00Z",
        "openid_email": null,
        "openid_enabled": false,
        "country_id": null,
        "intercom_hash": null,
        "has_password": true,
        "options": null,
        "tags": null
    });

    with_mockito(
        Method::POST,
        "/me/lost_password/token",
        200,
        Some(response),
        |client| {
            let user = client.me().reset_token(&reset_request)?;
            assert_eq!(user.id, UserId(98765));
            assert_eq!(user.api_token, Some("new-token-12345".to_string()));
            Ok(())
        },
    )
}
