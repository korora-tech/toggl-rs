#[cfg(test)]
mod tests {
    use super::super::*;
    use crate::models::profitability::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::Result;

    #[test]
    fn test_get_project_profitability() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "project_id": 123,
                    "total_tracked_seconds": 144000,
                    "total_billable_seconds": 129600,
                    "billable_amount": 5400.0,
                    "labor_cost": 3600.0,
                    "profit": 1800.0,
                    "profit_margin": 33.33,
                    "fixed_fee": null,
                    "currency": "USD"
                }
            ]
        });

        with_mockito(
            Method::POST,
            "/workspace/123/profitability/projects",
            200,
            Some(response),
            |client| {
                let request = ProjectProfitabilityRequest {
                    project_ids: vec![123],
                    filter: Some(ProfitabilityFilter {
                        start_date: Some("2024-01-01".to_string()),
                        end_date: Some("2024-01-31".to_string()),
                    }),
                };

                let report = client.profitability().projects(123, &request)?;
                assert_eq!(report.data.len(), 1);
                assert_eq!(report.data[0].project_id, 123);
                assert_eq!(report.data[0].billable_amount, 5400.0);
                assert_eq!(report.data[0].profit, 1800.0);
                assert_eq!(report.data[0].currency, "USD");
                Ok(())
            },
        )
    }
}
