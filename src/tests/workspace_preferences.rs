use crate::error::Result;
use crate::model::api::*;
use serde_json::json;

use super::with_mockito;
use reqwest::Method;

#[test]
fn test_get_workspace_preferences() -> Result<()> {
    let response = json!({
        "logo": "https://example.com/workspace-logo.png"
    });

    with_mockito(
        Method::GET,
        "/workspaces/123/preferences",
        200,
        Some(response),
        |client| {
            let logo = client.workspaces().get_preferences(123)?;
            assert_eq!(logo.logo, "https://example.com/workspace-logo.png");
            Ok(())
        },
    )
}

#[test]
fn test_update_workspace_preferences() -> Result<()> {
    let preferences = WorkspacePreferences {
        disable_approvals: Some(true),
        disable_expenses: Some(false),
        disable_timesheet_view: Some(false),
        hide_start_end_times: Some(true),
        inc_tos_accepted_at: None,
        inc_tos_accepted_by: None,
        initial_pricing_plan: None,
        report_locked_at: Some("2024-01-01T00:00:00Z".to_string()),
        single_sign_on: Some(true),
        sso_requested_at: Some("2024-01-01T00:00:00Z".to_string()),
    };

    let response = json!({
        "logo": "https://example.com/updated-logo.png"
    });

    with_mockito(
        Method::POST,
        "/workspaces/123/preferences",
        200,
        Some(response),
        |client| {
            let logo = client.workspaces().update_preferences(123, &preferences)?;
            assert_eq!(logo.logo, "https://example.com/updated-logo.png");
            Ok(())
        },
    )
}
