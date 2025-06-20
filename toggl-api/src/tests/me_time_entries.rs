#[cfg(test)]
mod tests {
    use crate::{models::api::time_entry::*, tests::with_mockito};
    use chrono::Utc;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;
    use toggl_core::{ProjectId, TimeEntryId, WorkspaceId};

    #[test]
    fn test_get_time_entries() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "workspace_id": 123,
                "user_id": 456,
                "billable": false,
                "start": "2023-01-01T10:00:00Z",
                "stop": "2023-01-01T11:00:00Z",
                "duration": 3600,
                "description": "Working on project",
                "project_id": 789,
                "task_id": null,
                "tag_ids": null,
                "tags": null,
                "at": "2023-01-01T10:00:00Z"
            }
        ]);

        with_mockito(
            Method::GET,
            "/me/time_entries",
            200,
            Some(response),
            |client| {
                let entries = client.me().get_time_entries(None, None, None, None, None)?;
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].id, TimeEntryId(1));
                assert_eq!(
                    entries[0].description.as_ref().unwrap(),
                    "Working on project"
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_time_entry() -> Result<()> {
        let create_data = CreateTimeEntry {
            workspace_id: WorkspaceId(123),
            start: Utc::now(),
            duration: 3600,
            description: Some("New task".to_string()),
            project_id: Some(ProjectId(789)),
            task_id: None,
            tag_ids: None,
            billable: Some(false),
            created_with: "toggl-rs".to_string(),
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": false,
            "start": "2023-01-01T10:00:00Z",
            "stop": "2023-01-01T11:00:00Z",
            "duration": 3600,
            "description": "New task",
            "project_id": 789,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::POST,
            "/me/time_entries",
            200,
            Some(response),
            |client| {
                let entry = client.me().create_time_entry(&create_data)?;
                assert_eq!(entry.id, TimeEntryId(1));
                assert_eq!(entry.description.as_ref().unwrap(), "New task");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_time_entry() -> Result<()> {
        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": false,
            "start": "2023-01-01T10:00:00Z",
            "stop": "2023-01-01T11:00:00Z",
            "duration": 3600,
            "description": "Working on project",
            "project_id": 789,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::GET,
            "/me/time_entries/1",
            200,
            Some(response),
            |client| {
                let entry = client.me().get_time_entry(TimeEntryId(1))?;
                assert_eq!(entry.id, TimeEntryId(1));
                assert_eq!(entry.description.as_ref().unwrap(), "Working on project");
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_time_entry() -> Result<()> {
        let update_data = UpdateTimeEntry {
            description: Some("Updated task".to_string()),
            project_id: Some(ProjectId(999)),
            task_id: None,
            tag_ids: None,
            billable: Some(true),
            start: None,
            stop: None,
            duration: None,
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": true,
            "start": "2023-01-01T10:00:00Z",
            "stop": "2023-01-01T11:00:00Z",
            "duration": 3600,
            "description": "Updated task",
            "project_id": 999,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::PUT,
            "/me/time_entries/1",
            200,
            Some(response),
            |client| {
                let entry = client
                    .me()
                    .update_time_entry(TimeEntryId(1), &update_data)?;
                assert_eq!(entry.id, TimeEntryId(1));
                assert_eq!(entry.description.as_ref().unwrap(), "Updated task");
                assert!(entry.billable);
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_time_entry() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/me/time_entries/1",
            200,
            None::<serde_json::Value>,
            |client| {
                client.me().delete_time_entry(TimeEntryId(1))?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_time_entries_checklist() -> Result<()> {
        let response = json!({
            "errors": [
                {
                    "code": "missing_project",
                    "message": "Time entry is missing project",
                    "time_entry_ids": [1, 2, 3]
                }
            ],
            "warnings": [
                {
                    "code": "long_duration",
                    "message": "Time entry has unusually long duration",
                    "time_entry_ids": [4, 5]
                }
            ]
        });

        with_mockito(
            Method::GET,
            "/me/time_entries/checklist",
            200,
            Some(response),
            |client| {
                let checklist = client.me().get_time_entries_checklist()?;
                assert_eq!(checklist.errors.len(), 1);
                assert_eq!(checklist.warnings.len(), 1);
                assert_eq!(checklist.errors[0].code, "missing_project");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_current_time_entry() -> Result<()> {
        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": false,
            "start": "2023-01-01T10:00:00Z",
            "stop": null,
            "duration": -1672570800,
            "description": "Currently tracking",
            "project_id": 789,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::GET,
            "/me/time_entries/current",
            200,
            Some(response),
            |client| {
                let entry = client.me().get_current_time_entry()?;
                assert!(entry.is_some());
                let entry = entry.unwrap();
                assert_eq!(entry.id, TimeEntryId(1));
                assert_eq!(entry.description.as_ref().unwrap(), "Currently tracking");
                assert!(entry.stop.is_none());
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_current_time_entry_none() -> Result<()> {
        with_mockito(
            Method::GET,
            "/me/time_entries/current",
            200,
            Some(serde_json::Value::Null),
            |client| {
                let entry = client.me().get_current_time_entry()?;
                assert!(entry.is_none());
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_current_time_entry() -> Result<()> {
        let update_data = UpdateTimeEntry {
            description: Some("Updated current".to_string()),
            project_id: None,
            task_id: None,
            tag_ids: None,
            billable: None,
            start: None,
            stop: None,
            duration: None,
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": false,
            "start": "2023-01-01T10:00:00Z",
            "stop": null,
            "duration": -1672570800,
            "description": "Updated current",
            "project_id": 789,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::PUT,
            "/me/time_entries/current",
            200,
            Some(response),
            |client| {
                let entry = client.me().update_current_time_entry(&update_data)?;
                assert_eq!(entry.id, TimeEntryId(1));
                assert_eq!(entry.description.as_ref().unwrap(), "Updated current");
                Ok(())
            },
        )
    }

    #[test]
    fn test_stop_current_time_entry() -> Result<()> {
        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "user_id": 456,
            "billable": false,
            "start": "2023-01-01T10:00:00Z",
            "stop": "2023-01-01T11:00:00Z",
            "duration": 3600,
            "description": "Stopped tracking",
            "project_id": 789,
            "task_id": null,
            "tag_ids": null,
            "tags": null,
            "at": "2023-01-01T10:00:00Z"
        });

        with_mockito(
            Method::DELETE,
            "/me/time_entries/current",
            200,
            Some(response),
            |client| {
                let entry = client.me().stop_current_time_entry()?;
                assert_eq!(entry.id, TimeEntryId(1));
                assert!(entry.stop.is_some());
                Ok(())
            },
        )
    }
}
