#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::base::*;
    use crate::models::comparative::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{Result, WorkspaceId};

    #[test]
    fn test_get_comparative_report() -> Result<()> {
        let response = json!({
            "data": {
                "graph": [
                    {
                        "date": "2024-01-01",
                        "current": {
                            "seconds": 28800,
                            "formatted": "8:00:00"
                        },
                        "previous": {
                            "seconds": 25200,
                            "formatted": "7:00:00"
                        }
                    },
                    {
                        "date": "2024-01-02",
                        "current": {
                            "seconds": 32400,
                            "formatted": "9:00:00"
                        },
                        "previous": {
                            "seconds": 28800,
                            "formatted": "8:00:00"
                        }
                    }
                ],
                "total_current": 61200,
                "total_previous": 54000,
                "change_percentage": 13.33,
                "resolution": "day"
            }
        });

        with_mockito(
            Method::POST,
            "/workspace/123/comparative",
            200,
            Some(response),
            |client| {
                let request = ComparativePost {
                    range: RangePost {
                        start_date: "2024-01-01".to_string(),
                        end_date: "2024-01-31".to_string(),
                        previous_start_date: Some("2023-12-01".to_string()),
                        previous_end_date: Some("2023-12-31".to_string()),
                    },
                    project_ids: None,
                    user_ids: None,
                    resolution: Some("day".to_string()),
                    include_time_entry_ids: None,
                };

                let report = client.comparative().get(WorkspaceId(123), &request)?;
                assert_eq!(report.data.graph.len(), 2);
                assert_eq!(report.data.total_current, 61200);
                assert_eq!(report.data.total_previous, 54000);
                assert_eq!(report.data.resolution, "day");
                Ok(())
            },
        )
    }
}
