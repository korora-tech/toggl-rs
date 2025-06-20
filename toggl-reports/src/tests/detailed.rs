#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::base::*;
    use crate::models::detailed::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{Result, TimeEntryId, WorkspaceId};

    #[test]
    fn test_get_detailed_report() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 1,
                    "user_id": 123,
                    "project_id": 456,
                    "task_id": null,
                    "client_id": 789,
                    "description": "Working on feature",
                    "billable": true,
                    "start": "2024-01-01T09:00:00Z",
                    "stop": "2024-01-01T11:00:00Z",
                    "duration": 7200,
                    "tag_ids": [1, 2],
                    "amount": 100.0,
                    "currency": "USD"
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/search/time_entries",
            200,
            Some(response),
            |client| {
                let request = DetailedPost {
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
                    order_by: None,
                    order_dir: None,
                    page: None,
                    per_page: None,
                    grouping: None,
                    include_time_entry_ids: None,
                    hide_amounts: None,
                };

                let report = client.detailed().get(WorkspaceId(123), &request)?;
                if let DetailedReportData::Single(entries) = report.data {
                    assert_eq!(entries.len(), 1);
                    assert_eq!(entries[0].id, TimeEntryId(1));
                    assert_eq!(entries[0].duration, 7200);
                } else {
                    panic!("Expected single entries");
                }
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_time_entries_totals() -> Result<()> {
        let response = json!({
            "time": 28800,
            "amount": 400.0,
            "count": 4
        });

        with_mockito(
            Method::POST,
            "/workspace/123/search/time_entries/totals",
            200,
            Some(response),
            |client| {
                let request = DetailedPost {
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
                    order_by: None,
                    order_dir: None,
                    page: None,
                    per_page: None,
                    grouping: None,
                    include_time_entry_ids: None,
                    hide_amounts: None,
                };

                let totals = client.detailed().totals(WorkspaceId(123), &request)?;
                assert_eq!(totals.time, 28800);
                assert_eq!(totals.count, 4);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_detailed_csv() -> Result<()> {
        let csv_data =
            b"Date,Description,Duration\n2024-01-01,Working on feature,02:00:00".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/search/time_entries.csv",
            200,
            csv_data.clone(),
            |client| {
                let request = SearchExportPost {
                    detailed: DetailedPost {
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
                        order_by: None,
                        order_dir: None,
                        page: None,
                        per_page: None,
                        grouping: None,
                        include_time_entry_ids: None,
                        hide_amounts: None,
                    },
                    extension: Some("csv".to_string()),
                };

                let data = client.detailed().export_csv(WorkspaceId(123), &request)?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_detailed_xlsx() -> Result<()> {
        let xlsx_data = vec![0x50, 0x4B, 0x03, 0x04]; // XLSX magic bytes

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/search/time_entries.xlsx",
            200,
            xlsx_data.clone(),
            |client| {
                let request = SearchExportPost {
                    detailed: DetailedPost {
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
                        order_by: None,
                        order_dir: None,
                        page: None,
                        per_page: None,
                        grouping: None,
                        include_time_entry_ids: None,
                        hide_amounts: None,
                    },
                    extension: Some("xlsx".to_string()),
                };

                let data = client.detailed().export_xlsx(WorkspaceId(123), &request)?;
                assert_eq!(&data[0..4], &[0x50, 0x4B, 0x03, 0x04]);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_detailed_pdf() -> Result<()> {
        let pdf_data = vec![0x25, 0x50, 0x44, 0x46]; // PDF magic bytes

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/search/time_entries.pdf",
            200,
            pdf_data.clone(),
            |client| {
                let request = ExportPDFPost {
                    detailed: DetailedPost {
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
                        order_by: None,
                        order_dir: None,
                        page: None,
                        per_page: None,
                        grouping: None,
                        include_time_entry_ids: None,
                        hide_amounts: None,
                    },
                    display_mode: None,
                    date_format: None,
                    duration_format: None,
                };

                let data = client.detailed().export_pdf(WorkspaceId(123), &request)?;
                assert_eq!(&data[0..4], &[0x25, 0x50, 0x44, 0x46]);
                Ok(())
            },
        )
    }
}
