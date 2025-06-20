#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::DataTrendsPost;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{ProjectId, Result, UserId, WorkspaceId};

    #[test]
    fn test_get_client_data_trends() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "client_id": 123,
                    "client_name": "Client ABC",
                    "graph": [
                        {
                            "date": "2024-01-01",
                            "seconds": 28800
                        },
                        {
                            "date": "2024-01-02",
                            "seconds": 32400
                        }
                    ],
                    "total_seconds": 61200
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/data_trends/clients",
            200,
            Some(response),
            |client| {
                let request = DataTrendsPost {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: None,
                    client_ids: None,
                    user_ids: None,
                    billable: None,
                };

                let report = client.data_trends().clients(WorkspaceId(123), &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.data[0].client_name.as_ref().unwrap(), "Client ABC");
                assert_eq!(report.data[0].total_seconds, 61200);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_project_data_trends() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "project_id": 456,
                    "project_name": "Project Alpha",
                    "client_id": 123,
                    "client_name": "Client ABC",
                    "graph": [
                        {
                            "date": "2024-01-01",
                            "seconds": 14400
                        }
                    ],
                    "total_seconds": 14400
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/data_trends/projects",
            200,
            Some(response),
            |client| {
                let request = DataTrendsPost {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: Some(vec![ProjectId(456)]),
                    client_ids: None,
                    user_ids: None,
                    billable: None,
                };

                let report = client.data_trends().projects(WorkspaceId(123), &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.data[0].project_name, "Project Alpha");
                assert_eq!(report.data[0].total_seconds, 14400);
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_user_data_trends() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "user_id": 789,
                    "user_name": "John Doe",
                    "graph": [
                        {
                            "date": "2024-01-01",
                            "seconds": 28800
                        }
                    ],
                    "total_seconds": 28800
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/data_trends/users",
            200,
            Some(response),
            |client| {
                let request = DataTrendsPost {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: None,
                    client_ids: None,
                    user_ids: Some(vec![UserId(789)]),
                    billable: None,
                };

                let report = client.data_trends().users(WorkspaceId(123), &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.data[0].user_name, "John Doe");
                assert_eq!(report.data[0].total_seconds, 28800);
                Ok(())
            },
        )
    }
}
