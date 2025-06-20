#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::filters::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;

    #[test]
    fn test_filter_clients() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "Client ABC",
                "archived": false
            },
            {
                "id": 2,
                "name": "Client XYZ",
                "archived": false
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/clients",
            200,
            Some(response),
            |client| {
                let request = ClientFilterParamsRequest {
                    name: Some("Client".to_string()),
                    status: None,
                };

                let result = client.filters().clients(123, &request)?;
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].name, "Client ABC");
                assert!(!result[0].archived);
                Ok(())
            },
        )
    }

    #[test]
    fn test_filter_projects() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "Project Alpha",
                "client_id": 123,
                "active": true,
                "billable": true,
                "color": "#ff0000"
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/projects",
            200,
            Some(response),
            |client| {
                let request = ProjectFilterParamRequest {
                    name: Some("Alpha".to_string()),
                    client_ids: None,
                    is_active: Some(true),
                    is_billable: None,
                    page: None,
                    per_page: None,
                };

                let result = client.filters().projects(123, &request)?;
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].name, "Project Alpha");
                assert!(result[0].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_filter_users() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "John Doe",
                "email": "john@example.com",
                "active": true
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/users",
            200,
            Some(response),
            |client| {
                let request = UserFilterParamsRequest {
                    name: Some("John".to_string()),
                    is_active: Some(true),
                };

                let result = client.filters().users(123, &request)?;
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].name, "John Doe");
                assert!(result[0].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_project_status() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "active": true,
                "billable": true
            },
            {
                "id": 2,
                "active": false,
                "billable": false
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/projects/status",
            200,
            Some(response),
            |client| {
                let request = ProjectStatusParamsRequest {
                    project_ids: vec![1, 2],
                };

                let result = client.filters().project_status(123, &request)?;
                assert_eq!(result.len(), 2);
                assert!(result[0].active);
                assert!(!result[1].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_project_groups() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "Group Alpha",
                "project_ids": [1, 2, 3]
            },
            {
                "id": 2,
                "name": "Group Beta",
                "project_ids": [4, 5]
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/project_groups",
            200,
            Some(response),
            |client| {
                let request = ProjectGroupParamsRequest {
                    name: Some("Group".to_string()),
                    project_ids: None,
                };

                let result = client.filters().project_groups(123, &request)?;
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].name, "Group Alpha");
                assert_eq!(result[0].project_ids, vec![1, 2, 3]);
                Ok(())
            },
        )
    }

    #[test]
    fn test_project_users() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "John Doe",
                "email": "john@example.com"
            },
            {
                "id": 2,
                "name": "Jane Smith",
                "email": "jane@example.com"
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/project_users",
            200,
            Some(response),
            |client| {
                let request = ProjectUserParamsRequest {
                    name: Some("John".to_string()),
                    project_ids: Some(vec![1]),
                };

                let result = client.filters().project_users(123, &request)?;
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].name, "John Doe");
                assert_eq!(result[0].id, 1);
                Ok(())
            },
        )
    }

    #[test]
    fn test_task_status() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "active": true
            },
            {
                "id": 2,
                "active": false
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/tasks/status",
            200,
            Some(response),
            |client| {
                let request = TaskStatusParamsRequest {
                    task_ids: vec![1, 2],
                };

                let result = client.filters().task_status(123, &request)?;
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].id, 1);
                assert!(result[0].active);
                assert!(!result[1].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_tasks_search() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "name": "Task Alpha",
                "active": true,
                "project_id": 1,
                "project_name": "Project Alpha",
                "client_name": "Client ABC",
                "estimated_seconds": 3600,
                "project_color": "#ff0000",
                "project_billable": true
            },
            {
                "id": 2,
                "name": "Task Beta",
                "active": false,
                "project_id": 2,
                "project_name": "Project Beta",
                "client_name": "Client XYZ",
                "estimated_seconds": 7200,
                "project_color": "#00ff00",
                "project_billable": false
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/search/tasks",
            200,
            Some(response),
            |client| {
                let request = TasksRequest {
                    active: Some(true),
                    ids: None,
                    name: Some("Task".to_string()),
                    page_size: Some(10),
                    project_active: None,
                    project_ids: Some(vec![1, 2]),
                    start: None,
                    user_ids: None,
                };

                let result = client.filters().tasks(123, "search", &request)?;
                assert_eq!(result.len(), 2);
                assert_eq!(result[0].name, "Task Alpha");
                assert_eq!(result[0].project_name, Some("Project Alpha".to_string()));
                assert!(result[0].active);
                assert!(!result[1].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_tasks_filters() -> Result<()> {
        let response = json!([
            {
                "id": 3,
                "name": "Task Gamma",
                "active": true,
                "project_id": 3,
                "project_name": "Project Gamma",
                "estimated_seconds": 1800
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspace/123/filters/tasks",
            200,
            Some(response),
            |client| {
                let request = TasksRequest {
                    active: Some(true),
                    ids: Some(vec![3]),
                    name: None,
                    page_size: None,
                    project_active: Some(true),
                    project_ids: None,
                    start: None,
                    user_ids: None,
                };

                let result = client.filters().tasks(123, "filters", &request)?;
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].name, "Task Gamma");
                assert_eq!(result[0].id, 3);
                assert!(result[0].active);
                Ok(())
            },
        )
    }
}
