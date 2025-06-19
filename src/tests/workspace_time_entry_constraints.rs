use crate::error::Result;
use crate::model::api::preferences::TimeEntryConstraints;
use crate::tests::*;
use reqwest::Method;
use serde_json::json;

#[test]
fn test_get_time_entry_constraints() -> Result<()> {
    let workspace_id = 12345;

    let response = json!({
        "description_required": true,
        "project_required": false,
        "tag_required": true,
        "task_required": false,
        "time_entry_constraints_enabled": true
    });

    with_mockito(
        Method::GET,
        &format!("/workspaces/{}/time_entry_constraints", workspace_id),
        200,
        Some(response),
        |client| {
            let constraints = client
                .workspaces()
                .get_time_entry_constraints(workspace_id)?;
            assert!(constraints.description_required);
            assert!(!constraints.project_required);
            assert!(constraints.tag_required);
            assert!(!constraints.task_required);
            assert!(constraints.time_entry_constraints_enabled);
            Ok(())
        },
    )
}

#[test]
fn test_create_time_entry_constraints() -> Result<()> {
    let workspace_id = 12345;

    let constraints = TimeEntryConstraints {
        description_required: true,
        project_required: true,
        tag_required: false,
        task_required: false,
        time_entry_constraints_enabled: true,
    };

    let response = json!({
        "description_required": true,
        "project_required": true,
        "tag_required": false,
        "task_required": false,
        "time_entry_constraints_enabled": true
    });

    with_mockito(
        Method::POST,
        &format!("/workspaces/{}/time_entry_constraints", workspace_id),
        200,
        Some(response),
        |client| {
            let result = client
                .workspaces()
                .create_time_entry_constraints(workspace_id, &constraints)?;
            assert!(result.description_required);
            assert!(result.project_required);
            assert!(!result.tag_required);
            assert!(!result.task_required);
            assert!(result.time_entry_constraints_enabled);
            Ok(())
        },
    )
}

#[test]
fn test_update_time_entry_constraints() -> Result<()> {
    let workspace_id = 12345;

    let constraints = TimeEntryConstraints {
        description_required: false,
        project_required: true,
        tag_required: true,
        task_required: true,
        time_entry_constraints_enabled: true,
    };

    let response = json!({
        "description_required": false,
        "project_required": true,
        "tag_required": true,
        "task_required": true,
        "time_entry_constraints_enabled": true
    });

    with_mockito(
        Method::PATCH,
        &format!("/workspaces/{}/time_entry_constraints", workspace_id),
        200,
        Some(response),
        |client| {
            let result = client
                .workspaces()
                .update_time_entry_constraints(workspace_id, &constraints)?;
            assert!(!result.description_required);
            assert!(result.project_required);
            assert!(result.tag_required);
            assert!(result.task_required);
            assert!(result.time_entry_constraints_enabled);
            Ok(())
        },
    )
}

#[test]
fn test_update_time_entry_constraints_disabled() -> Result<()> {
    let workspace_id = 12345;

    let constraints = TimeEntryConstraints {
        description_required: false,
        project_required: false,
        tag_required: false,
        task_required: false,
        time_entry_constraints_enabled: false,
    };

    let response = json!({
        "description_required": false,
        "project_required": false,
        "tag_required": false,
        "task_required": false,
        "time_entry_constraints_enabled": false
    });

    with_mockito(
        Method::PATCH,
        &format!("/workspaces/{}/time_entry_constraints", workspace_id),
        200,
        Some(response),
        |client| {
            let result = client
                .workspaces()
                .update_time_entry_constraints(workspace_id, &constraints)?;
            assert!(!result.description_required);
            assert!(!result.project_required);
            assert!(!result.tag_required);
            assert!(!result.task_required);
            assert!(!result.time_entry_constraints_enabled);
            Ok(())
        },
    )
}
