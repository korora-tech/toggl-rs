#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::base::*;
    use crate::models::summary::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{Result, WorkspaceId};

    #[test]
    fn test_get_summary_report() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 123,
                    "time": 7200,
                    "title": "Project A",
                    "project_ids": [123],
                    "amount": 100.0,
                    "currency": "USD"
                }
            ],
            "totals": {
                "time": 7200,
                "amount": 100.0,
                "currency": "USD"
            }
        });

        with_mockito(
            Method::POST,
            "/workspace/123/summary/time_entries",
            200,
            Some(response),
            |client| {
                let request = ReportPost {
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
                    grouping: Some(GroupingOption::Projects),
                    sub_grouping: None,
                    order_by: None,
                    order_dir: None,
                    include_time_entry_ids: None,
                    distinguish_rates: None,
                    resolution: None,
                    hide_amounts: None,
                };

                let report = client.summary().get(WorkspaceId(123), &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.totals.time, 7200);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_summary_csv() -> Result<()> {
        let csv_data = b"Project,Time,Amount\nProject A,02:00:00,100.00".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/summary/time_entries.csv",
            200,
            csv_data.clone(),
            |client| {
                let request = ExportPost {
                    report: ReportPost {
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
                        sub_grouping: None,
                        order_by: None,
                        order_dir: None,
                        include_time_entry_ids: None,
                        distinguish_rates: None,
                        resolution: None,
                        hide_amounts: None,
                    },
                    extension: Some("csv".to_string()),
                };

                let data = client.summary().export_csv(WorkspaceId(123), &request)?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_summary_xlsx() -> Result<()> {
        let xlsx_data = vec![0x50, 0x4B, 0x03, 0x04]; // XLSX magic bytes

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/summary/time_entries.xlsx",
            200,
            xlsx_data.clone(),
            |client| {
                let request = ExportPost {
                    report: ReportPost {
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
                        sub_grouping: None,
                        order_by: None,
                        order_dir: None,
                        include_time_entry_ids: None,
                        distinguish_rates: None,
                        resolution: None,
                        hide_amounts: None,
                    },
                    extension: Some("xlsx".to_string()),
                };

                let data = client.summary().export_xlsx(WorkspaceId(123), &request)?;
                assert_eq!(&data[0..4], &[0x50, 0x4B, 0x03, 0x04]);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_summary_pdf() -> Result<()> {
        let pdf_data = b"%PDF-1.4".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/workspace/123/summary/time_entries.pdf",
            200,
            pdf_data.clone(),
            |client| {
                let request = ExportPDFPost {
                    report: ReportPost {
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
                        sub_grouping: None,
                        order_by: None,
                        order_dir: None,
                        include_time_entry_ids: None,
                        distinguish_rates: None,
                        resolution: None,
                        hide_amounts: None,
                    },
                    display_mode: Some("compact".to_string()),
                };

                let data = client.summary().export_pdf(WorkspaceId(123), &request)?;
                assert!(data.starts_with(b"%PDF"));
                Ok(())
            },
        )
    }
}
