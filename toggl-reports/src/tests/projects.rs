#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::projects::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{ProjectId, Result, WorkspaceId};

    #[test]
    fn test_get_projects_summary() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "project_id": 123,
                    "project_name": "Project Alpha",
                    "client_id": 456,
                    "client_name": "Client ABC",
                    "tracked_seconds": 144000,
                    "billable_seconds": 129600,
                    "billable_amount": 5400.0,
                    "labor_cost": 3600.0,
                    "currency": "USD"
                },
                {
                    "project_id": 789,
                    "project_name": "Project Beta",
                    "client_id": null,
                    "client_name": null,
                    "tracked_seconds": 72000,
                    "billable_seconds": 72000,
                    "billable_amount": 3000.0,
                    "labor_cost": 2000.0,
                    "currency": "USD"
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/projects/summary",
            200,
            Some(response),
            |client| {
                let request = ProjectSummaryRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    include_time_entry_ids: None,
                };

                let result = client.projects().summary(WorkspaceId(123), &request)?;
                assert_eq!(result.data.len(), 2);
                assert_eq!(result.data[0].project_name, "Project Alpha");
                assert_eq!(result.data[0].tracked_seconds, 144000);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_project_summary() -> Result<()> {
        let response = json!({
            "project_id": 123,
            "tracked_seconds": 144000,
            "billable_seconds": 129600,
            "billable_amount": 5400.0,
            "labor_cost": 3600.0,
            "currency": "USD",
            "time_entry_ids": [1, 2, 3, 4, 5]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/projects/456/summary",
            200,
            Some(response),
            |client| {
                let request = ProjectSummaryRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    include_time_entry_ids: Some(true),
                };

                let result = client
                    .projects()
                    .project_summary(WorkspaceId(123), 456, &request)?;
                assert_eq!(result.project_id, ProjectId(123));
                assert_eq!(result.tracked_seconds, 144000);
                assert!(result.time_entry_ids.is_some());
                assert_eq!(result.time_entry_ids.unwrap().len(), 5);
                Ok(())
            },
        )
    }
}
