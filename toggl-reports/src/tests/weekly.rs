#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::base::*;
    use crate::models::weekly::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;

    #[test]
    fn test_get_weekly_report() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "user_id": 123,
                    "user": {
                        "id": 123,
                        "name": "John Doe",
                        "email": "john@example.com"
                    },
                    "details": [
                        {
                            "id": 456,
                            "title": "Project A",
                            "time_entries": [
                                {
                                    "week_start": "2024-01-01",
                                    "seconds": 28800
                                }
                            ],
                            "total_seconds": 28800
                        }
                    ],
                    "total_seconds": 28800,
                    "total_billable_seconds": 25200
                }
            ],
            "week_totals": [
                {
                    "week_start": "2024-01-01",
                    "seconds": 28800
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/weekly/time_entries",
            200,
            Some(response),
            |client| {
                let request = WeeklyPost {
                    base: Post {
                        start_date: "2024-01-01".to_string(),
                        end_date: "2024-01-31".to_string(),
                        client_ids: None,
                        project_ids: None,
                        user_ids: None,
                        tag_ids: None,
                        task_ids: None,
                        billable: None,
                        description: None,
                        min_duration_seconds: None,
                        max_duration_seconds: None,
                        rounding: None,
                        rounding_minutes: None,
                    },
                    grouping: Some("projects".to_string()),
                    calculate: None,
                };

                let report = client.weekly().get(123, &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.data[0].user_id, 123);
                assert_eq!(report.data[0].total_seconds, 28800);
                assert_eq!(report.week_totals.len(), 1);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_weekly_csv() -> Result<()> {
        let csv_data = b"User,Week,Hours\nJohn Doe,2024-01-01,8.0".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/weekly/time_entries.csv",
            200,
            csv_data.clone(),
            |client| {
                let request = WeeklyExportPost {
                    weekly: WeeklyPost {
                        base: Post {
                            start_date: "2024-01-01".to_string(),
                            end_date: "2024-01-31".to_string(),
                            client_ids: None,
                            project_ids: None,
                            user_ids: None,
                            tag_ids: None,
                            task_ids: None,
                            billable: None,
                            description: None,
                            min_duration_seconds: None,
                            max_duration_seconds: None,
                            rounding: None,
                            rounding_minutes: None,
                        },
                        grouping: None,
                        calculate: None,
                    },
                };

                let data = client.weekly().export_csv(123, &request)?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_weekly_pdf() -> Result<()> {
        let pdf_data = vec![0x25, 0x50, 0x44, 0x46]; // PDF magic bytes

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/weekly/time_entries.pdf",
            200,
            pdf_data.clone(),
            |client| {
                let request = WeeklyExportPDFPost {
                    weekly: WeeklyPost {
                        base: Post {
                            start_date: "2024-01-01".to_string(),
                            end_date: "2024-01-31".to_string(),
                            client_ids: None,
                            project_ids: None,
                            user_ids: None,
                            tag_ids: None,
                            task_ids: None,
                            billable: None,
                            description: None,
                            min_duration_seconds: None,
                            max_duration_seconds: None,
                            rounding: None,
                            rounding_minutes: None,
                        },
                        grouping: None,
                        calculate: None,
                    },
                    display_mode: Some("compact".to_string()),
                };

                let data = client.weekly().export_pdf(123, &request)?;
                assert_eq!(&data[0..4], &[0x25, 0x50, 0x44, 0x46]);
                Ok(())
            },
        )
    }
}
