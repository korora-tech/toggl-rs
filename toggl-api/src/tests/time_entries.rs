use crate::models::api::ids::{TimeEntryId, WorkspaceId};
use crate::models::api::time_entry::*;
use chrono::{DateTime, Utc};
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_get_current_time_entry() -> Result<()> {
    let response = json!({
        "id": 1234567890,
        "workspace_id": 1234567,
        "project_id": null,
        "task_id": null,
        "billable": false,
        "start": "2023-01-01T12:00:00+00:00",
        "stop": null,
        "duration": -1672574400,
        "description": "Working on important task",
        "tags": ["work", "important"],
        "tag_ids": [123, 456],
        "duronly": false,
        "at": "2023-01-01T12:00:00+00:00",
        "server_deleted_at": null,
        "user_id": 123456,
        "uid": 123456,
        "wid": 1234567,
        "pid": null
    });

    with_mockito(
        Method::GET,
        "/me/time_entries/current",
        200,
        Some(response),
        |client| {
            let entry = client.time_entries().current()?;
            assert!(entry.is_some());
            let entry = entry.unwrap();
            assert_eq!(TimeEntryId(1234567890), entry.id);
            assert_eq!(
                Some("Working on important task".to_string()),
                entry.description
            );
            assert_eq!(2, entry.tags.as_ref().unwrap().len());
            Ok(())
        },
    )
}

#[test]
fn test_create_time_entry() -> Result<()> {
    let create_data = CreateTimeEntry {
        created_with: "toggl-rs".to_string(),
        description: Some("New time entry".to_string()),
        workspace_id: WorkspaceId(1234567),
        project_id: None,
        task_id: None,
        billable: Some(false),
        start: DateTime::parse_from_rfc3339("2023-01-01T14:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        duration: 3600,
        tag_ids: None,
    };

    let response = json!({
        "id": 987654321,
        "workspace_id": 1234567,
        "project_id": null,
        "task_id": null,
        "billable": false,
        "start": "2023-01-01T14:00:00+00:00",
        "stop": "2023-01-01T15:00:00+00:00",
        "duration": 3600,
        "description": "New time entry",
        "tags": ["test"],
        "tag_ids": [789],
        "duronly": false,
        "at": "2023-01-01T15:00:00+00:00",
        "server_deleted_at": null,
        "user_id": 123456,
        "uid": 123456,
        "wid": 1234567,
        "pid": null
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/time_entries",
        200,
        Some(response),
        |client| {
            let entry = client
                .time_entries()
                .create(WorkspaceId(1234567), &create_data)?;
            assert_eq!(TimeEntryId(987654321), entry.id);
            assert_eq!(Some("New time entry".to_string()), entry.description);
            assert_eq!(3600, entry.duration);
            Ok(())
        },
    )
}

#[test]
fn test_start_time_entry() -> Result<()> {
    let start_data = CreateTimeEntry {
        created_with: "toggl-rs".to_string(),
        description: Some("Starting work".to_string()),
        workspace_id: WorkspaceId(1234567),
        project_id: None,
        task_id: None,
        billable: Some(false),
        start: DateTime::parse_from_rfc3339("2023-01-01T16:00:00Z")
            .unwrap()
            .with_timezone(&Utc),
        duration: -1672588800,
        tag_ids: None,
    };

    let response = json!({
        "id": 1111111111,
        "workspace_id": 1234567,
        "project_id": null,
        "task_id": null,
        "billable": false,
        "start": "2023-01-01T16:00:00+00:00",
        "stop": null,
        "duration": -1672588800,
        "description": "Starting work",
        "tags": [],
        "tag_ids": [],
        "duronly": false,
        "at": "2023-01-01T16:00:00+00:00",
        "server_deleted_at": null,
        "user_id": 123456,
        "uid": 123456,
        "wid": 1234567,
        "pid": null
    });

    with_mockito(
        Method::POST,
        "/workspaces/1234567/time_entries/start",
        200,
        Some(response),
        |client| {
            let entry = client
                .time_entries()
                .start(WorkspaceId(1234567), &start_data)?;
            assert_eq!(TimeEntryId(1111111111), entry.id);
            assert_eq!(Some("Starting work".to_string()), entry.description);
            assert_eq!(-1672588800, entry.duration);
            Ok(())
        },
    )
}

#[test]
fn test_stop_time_entry() -> Result<()> {
    let response = json!({
        "id": 1111111111,
        "workspace_id": 1234567,
        "project_id": null,
        "task_id": null,
        "billable": false,
        "start": "2023-01-01T16:00:00+00:00",
        "stop": "2023-01-01T17:30:00+00:00",
        "duration": 5400,
        "description": "Starting work",
        "tags": [],
        "tag_ids": [],
        "duronly": false,
        "at": "2023-01-01T17:30:00+00:00",
        "server_deleted_at": null,
        "user_id": 123456,
        "uid": 123456,
        "wid": 1234567,
        "pid": null
    });

    with_mockito(
        Method::PATCH,
        "/workspaces/1234567/time_entries/1111111111/stop",
        200,
        Some(response),
        |client| {
            let entry = client
                .time_entries()
                .stop(WorkspaceId(1234567), TimeEntryId(1111111111))?;
            assert_eq!(TimeEntryId(1111111111), entry.id);
            assert_eq!(5400, entry.duration);
            assert!(entry.stop.is_some());
            Ok(())
        },
    )
}

#[test]
fn test_update_time_entry() -> Result<()> {
    let update_data = UpdateTimeEntry {
        description: Some("Updated description".to_string()),
        project_id: None,
        task_id: None,
        billable: Some(true),
        start: None,
        stop: None,
        duration: None,
        tag_ids: None,
    };

    let response = json!({
        "id": 1234567890,
        "workspace_id": 1234567,
        "project_id": null,
        "task_id": null,
        "billable": true,
        "start": "2023-01-01T12:00:00+00:00",
        "stop": "2023-01-01T13:00:00+00:00",
        "duration": 3600,
        "description": "Updated description",
        "tags": ["updated", "important"],
        "tag_ids": [123, 456],
        "duronly": false,
        "at": "2023-01-01T13:00:00+00:00",
        "server_deleted_at": null,
        "user_id": 123456,
        "uid": 123456,
        "wid": 1234567,
        "pid": null
    });

    with_mockito(
        Method::PUT,
        "/workspaces/1234567/time_entries/1234567890",
        200,
        Some(response),
        |client| {
            let entry = client.time_entries().update(
                WorkspaceId(1234567),
                TimeEntryId(1234567890),
                &update_data,
            )?;
            assert_eq!(Some("Updated description".to_string()), entry.description);
            assert_eq!(true, entry.billable);
            assert_eq!(2, entry.tags.as_ref().unwrap().len());
            Ok(())
        },
    )
}

#[test]
fn test_delete_time_entry() -> Result<()> {
    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/time_entries/1234567890",
        200,
        None,
        |client| {
            client
                .time_entries()
                .delete(WorkspaceId(1234567), TimeEntryId(1234567890))?;
            Ok(())
        },
    )
}

#[test]
fn test_list_time_entries() -> Result<()> {
    let response = json!([
        {
            "id": 1234567890,
            "workspace_id": 1234567,
            "project_id": null,
            "task_id": null,
            "billable": false,
            "start": "2023-01-01T09:00:00+00:00",
            "stop": "2023-01-01T10:00:00+00:00",
            "duration": 3600,
            "description": "Morning work",
            "tags": ["morning"],
            "tag_ids": [111],
            "duronly": false,
            "at": "2023-01-01T10:00:00+00:00",
            "server_deleted_at": null,
            "user_id": 123456,
            "uid": 123456,
            "wid": 1234567,
            "pid": null
        },
        {
            "id": 1234567891,
            "workspace_id": 1234567,
            "project_id": null,
            "task_id": null,
            "billable": false,
            "start": "2023-01-01T14:00:00+00:00",
            "stop": "2023-01-01T16:00:00+00:00",
            "duration": 7200,
            "description": "Afternoon work",
            "tags": ["afternoon"],
            "tag_ids": [222],
            "duronly": false,
            "at": "2023-01-01T16:00:00+00:00",
            "server_deleted_at": null,
            "user_id": 123456,
            "uid": 123456,
            "wid": 1234567,
            "pid": null
        }
    ]);

    with_mockito(
        Method::GET,
        "/me/time_entries",
        200,
        Some(response),
        |client| {
            let entries = client.time_entries().list(None, None)?;
            assert_eq!(2, entries.len());
            assert_eq!(Some("Morning work".to_string()), entries[0].description);
            assert_eq!(Some("Afternoon work".to_string()), entries[1].description);
            Ok(())
        },
    )
}

#[test]
fn test_bulk_delete_time_entries() -> Result<()> {
    let time_entry_ids = vec![
        TimeEntryId(123456),
        TimeEntryId(789012),
        TimeEntryId(345678),
    ];

    with_mockito(
        Method::DELETE,
        "/workspaces/1234567/time_entries/123456,789012,345678",
        200,
        None,
        |client| {
            client
                .time_entries()
                .bulk_delete(WorkspaceId(1234567), time_entry_ids)?;
            Ok(())
        },
    )
}

#[test]
fn test_get_time_entry_checklist() -> Result<()> {
    let response = json!({
        "errors": [
            {
                "code": "overlapping_time_entries",
                "message": "Time entries overlap",
                "time_entry_ids": [123456, 789012]
            }
        ],
        "warnings": [
            {
                "code": "missing_description",
                "message": "Time entries without description",
                "time_entry_ids": [345678]
            }
        ]
    });

    with_mockito(
        Method::GET,
        "/me/time_entries/checklist",
        200,
        Some(response),
        |client| {
            let checklist = client.time_entries().get_checklist()?;
            assert_eq!(1, checklist.errors.len());
            assert_eq!("overlapping_time_entries", checklist.errors[0].code);
            assert_eq!(1, checklist.warnings.len());
            assert_eq!("missing_description", checklist.warnings[0].code);
            Ok(())
        },
    )
}
