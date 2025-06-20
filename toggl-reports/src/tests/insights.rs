#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::profitability::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{ProjectId, Result, WorkspaceId};

    #[test]
    fn test_get_project_trends() -> Result<()> {
        let response = json!([
            {
                "project_id": 123,
                "trends": [
                    {
                        "period": "2024-01-01",
                        "total_seconds": 28800,
                        "billable_seconds": 25200
                    },
                    {
                        "period": "2024-01-02",
                        "total_seconds": 32400,
                        "billable_seconds": 32400
                    }
                ]
            }
        ]);

        with_mockito(
            Method::POST,
            "/api/v1/workspace/123/data_trends/projects",
            200,
            Some(response),
            |client| {
                let request = ProjectTrendsRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: Some(vec![ProjectId(123)]),
                    client_ids: None,
                    billable: None,
                    resolution: Some("day".to_string()),
                };

                let result = client
                    .insights()
                    .project_trends(WorkspaceId(123), &request)?;
                assert_eq!(result.len(), 1);
                assert_eq!(result[0].project_id, ProjectId(123));
                assert_eq!(result[0].trends.len(), 2);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_employee_profitability_csv() -> Result<()> {
        let csv_data = b"Employee,Hours,Billable Hours,Labor Cost,Revenue,Profit\nJohn Doe,160,144,6400,7200,800".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/api/v1/workspace/123/profitability/employees.csv",
            200,
            csv_data.clone(),
            |client| {
                let request = EmployeeProfitabilityRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    user_ids: None,
                    group_ids: None,
                    resolution: None,
                };

                let data = client
                    .insights()
                    .employee_profitability_csv(WorkspaceId(123), &request)?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_project_trends_xlsx() -> Result<()> {
        let xlsx_data = vec![0x50, 0x4B, 0x03, 0x04]; // XLSX magic bytes

        with_mockito_bytes(
            Method::POST,
            "/api/v1/workspace/123/trends/projects.xlsx",
            200,
            xlsx_data.clone(),
            |client| {
                let request = ProjectTrendsRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: None,
                    client_ids: None,
                    billable: None,
                    resolution: None,
                };

                let data = client
                    .insights()
                    .project_trends_xlsx(WorkspaceId(123), &request)?;
                assert_eq!(&data[0..4], &[0x50, 0x4B, 0x03, 0x04]);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_project_profitability_csv() -> Result<()> {
        let csv_data = b"Project,Hours,Billable Hours,Labor Cost,Revenue,Profit\nProject Alpha,160,144,6400,7200,800".to_vec();

        with_mockito_bytes(
            Method::POST,
            "/api/v1/workspace/123/profitability/projects.csv",
            200,
            csv_data.clone(),
            |client| {
                let request = ProjectTrendsRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: Some(vec![ProjectId(123)]),
                    client_ids: None,
                    billable: None,
                    resolution: None,
                };

                let data = client
                    .insights()
                    .project_profitability_csv(WorkspaceId(123), &request)?;
                assert_eq!(data, csv_data);
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_project_profitability_xlsx() -> Result<()> {
        let xlsx_data = vec![0x50, 0x4B, 0x03, 0x04]; // XLSX magic bytes

        with_mockito_bytes(
            Method::POST,
            "/api/v1/workspace/123/profitability/projects.xlsx",
            200,
            xlsx_data.clone(),
            |client| {
                let request = ProjectTrendsRequest {
                    start_date: "2024-01-01".to_string(),
                    end_date: "2024-01-31".to_string(),
                    project_ids: Some(vec![ProjectId(123)]),
                    client_ids: None,
                    billable: None,
                    resolution: None,
                };

                let data = client
                    .insights()
                    .project_profitability_xlsx(WorkspaceId(123), &request)?;
                assert_eq!(&data[0..4], &[0x50, 0x4B, 0x03, 0x04]);
                Ok(())
            },
        )
    }
}
