#[cfg(test)]
mod tests {
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn test_summary_report() -> Result<()> {
        let response = json!({
            "groups": [
                {
                    "id": 123,
                    "name": "Project A",
                    "duration": 7200,
                    "billable_duration": 3600,
                    "amount": 150.0,
                    "count": 5,
                    "sub_groups": null
                }
            ],
            "total": {
                "duration": 7200,
                "billable_duration": 3600,
                "amount": 150.0,
                "count": 5
            }
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/summary",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().summary(123456, params)?;

                assert_eq!(result.groups.len(), 1);
                assert_eq!(result.groups[0].id, Some(123));
                assert_eq!(result.groups[0].name, Some("Project A".to_string()));
                assert_eq!(result.groups[0].duration, 7200);
                assert_eq!(result.total.duration, 7200);
                assert_eq!(result.total.amount, Some(150.0));
                Ok(())
            },
        )
    }

    #[test]
    fn test_detailed_report() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 111,
                    "description": "Working on task",
                    "start": "2024-01-01T09:00:00Z",
                    "end": "2024-01-01T11:00:00Z",
                    "dur": 7200,
                    "user_name": "John Doe",
                    "user_id": 222,
                    "project_name": "Project A",
                    "project_id": 123,
                    "client_name": "Client X",
                    "client_id": 456,
                    "billable": true,
                    "is_billable": true,
                    "amount": 100.0,
                    "tags": [],
                    "task_id": null,
                    "task_name": null,
                    "project_color": "#1baada",
                    "rate": 50.0,
                    "cur": "USD"
                }
            ],
            "total_count": 1,
            "per_page": 50,
            "page": 1,
            "totals": {
                "time": 7200,
                "billable_time": 7200,
                "amount": 100.0,
                "billable_amount": 100.0,
                "count": 1
            }
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/detailed",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().detailed(123456, params)?;

                assert_eq!(result.data.len(), 1);
                assert_eq!(result.data[0].id, 111);
                assert_eq!(
                    result.data[0].description,
                    Some("Working on task".to_string())
                );
                assert_eq!(result.data[0].dur, 7200);
                assert_eq!(result.totals.time, 7200);
                Ok(())
            },
        )
    }

    #[test]
    fn test_weekly_report() -> Result<()> {
        let response = json!({
            "week_totals": [
                {
                    "week_start": "2024-01-01",
                    "week_end": "2024-01-07",
                    "seconds": 144000,
                    "billable_seconds": 72000,
                    "amount": 2000.0
                }
            ],
            "projects": []
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/weekly",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().weekly(123456, params)?;

                assert_eq!(result.week_totals.len(), 1);
                assert_eq!(result.week_totals[0].seconds, 144000);
                assert_eq!(result.week_totals[0].billable_seconds, Some(72000));
                assert_eq!(result.week_totals[0].amount, Some(2000.0));
                Ok(())
            },
        )
    }

    #[test]
    fn test_project_trends() -> Result<()> {
        let response = json!({
            "projects": [
                {
                    "project_id": 123,
                    "project_name": "Project A",
                    "client_id": null,
                    "client_name": null,
                    "color": "#1baada",
                    "data_points": [
                        {
                            "date": "2024-01-01",
                            "seconds": 144000
                        }
                    ],
                    "total_seconds": 144000
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-12-31");
        params.insert("granularity", "month");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/trends/projects",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-12-31".to_string());
                params.insert("granularity".to_string(), "month".to_string());

                let result = client.reports().project_trends(123456, params)?;

                assert_eq!(result.projects.len(), 1);
                assert_eq!(result.projects[0].project_id, Some(123));
                assert_eq!(
                    result.projects[0].project_name,
                    Some("Project A".to_string())
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_client_trends() -> Result<()> {
        let response = json!({
            "clients": [
                {
                    "client_id": 456,
                    "client_name": "Client X",
                    "data_points": [
                        {
                            "date": "2024-01-01",
                            "seconds": 288000
                        }
                    ],
                    "total_seconds": 288000
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-12-31");
        params.insert("granularity", "month");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/trends/clients",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-12-31".to_string());
                params.insert("granularity".to_string(), "month".to_string());

                let result = client.reports().client_trends(123456, params)?;

                assert_eq!(result.clients.len(), 1);
                assert_eq!(result.clients[0].client_id, Some(456));
                assert_eq!(result.clients[0].client_name, Some("Client X".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_user_trends() -> Result<()> {
        let response = json!({
            "users": [
                {
                    "user_id": 789,
                    "user_name": "John Doe",
                    "data_points": [
                        {
                            "date": "2024-01-01",
                            "seconds": 144000
                        }
                    ],
                    "total_seconds": 144000
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-12-31");
        params.insert("granularity", "month");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/trends/users",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-12-31".to_string());
                params.insert("granularity".to_string(), "month".to_string());

                let result = client.reports().user_trends(123456, params)?;

                assert_eq!(result.users.len(), 1);
                assert_eq!(result.users[0].user_id, 789);
                assert_eq!(result.users[0].user_name, "John Doe");
                Ok(())
            },
        )
    }

    #[test]
    fn test_project_profitability() -> Result<()> {
        let response = json!({
            "projects": [
                {
                    "project_id": 123,
                    "project_name": "Project A",
                    "client_id": 456,
                    "client_name": "Client X",
                    "billable_seconds": 144000,
                    "non_billable_seconds": 36000,
                    "total_seconds": 180000,
                    "billable_amount": 5000.0,
                    "labor_cost": 3000.0,
                    "profit": 2000.0,
                    "profit_margin": 0.4,
                    "fixed_fee": null,
                    "hourly_rate": 125.0
                }
            ],
            "totals": {
                "billable_seconds": 144000,
                "non_billable_seconds": 36000,
                "total_seconds": 180000,
                "billable_amount": 5000.0,
                "labor_cost": 3000.0,
                "profit": 2000.0,
                "profit_margin": 0.4
            }
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/profitability/projects",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().project_profitability(123456, params)?;

                assert_eq!(result.projects.len(), 1);
                assert_eq!(result.projects[0].project_id, 123);
                assert_eq!(result.projects[0].billable_amount, 5000.0);
                assert_eq!(result.projects[0].profit, 2000.0);
                assert_eq!(result.totals.profit_margin, 0.4);
                Ok(())
            },
        )
    }

    #[test]
    fn test_employee_profitability() -> Result<()> {
        let response = json!({
            "employees": [
                {
                    "user_id": 789,
                    "user_name": "John Doe",
                    "billable_seconds": 288000,
                    "non_billable_seconds": 72000,
                    "total_seconds": 360000,
                    "billable_amount": 10000.0,
                    "labor_cost": 6000.0,
                    "profit": 4000.0,
                    "profit_margin": 0.4,
                    "hourly_rate": 100.0
                }
            ],
            "totals": {
                "billable_seconds": 288000,
                "non_billable_seconds": 72000,
                "total_seconds": 360000,
                "billable_amount": 10000.0,
                "labor_cost": 6000.0,
                "profit": 4000.0,
                "profit_margin": 0.4
            }
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/profitability/employees",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().employee_profitability(123456, params)?;

                assert_eq!(result.employees.len(), 1);
                assert_eq!(result.employees[0].user_id, 789);
                assert_eq!(result.employees[0].billable_amount, 10000.0);
                assert_eq!(result.employees[0].profit, 4000.0);
                Ok(())
            },
        )
    }

    #[test]
    fn test_comparative_report() -> Result<()> {
        let response = json!({
            "base_period": {
                "start_date": "2024-01-01",
                "end_date": "2024-01-31",
                "total_seconds": 576000,
                "billable_seconds": 288000,
                "total_amount": 8000.0,
                "billable_amount": 8000.0,
                "projects": []
            },
            "comparison_period": {
                "start_date": "2024-02-01",
                "end_date": "2024-02-29",
                "total_seconds": 604800,
                "billable_seconds": 302400,
                "total_amount": 8400.0,
                "billable_amount": 8400.0,
                "projects": []
            },
            "change": {
                "total_seconds_change": 28800,
                "total_seconds_change_percentage": 0.05,
                "billable_seconds_change": 14400,
                "billable_seconds_change_percentage": 0.05,
                "total_amount_change": 400.0,
                "total_amount_change_percentage": 0.05
            }
        });

        let mut params = BTreeMap::new();
        params.insert("period_1_start", "2024-01-01");
        params.insert("period_1_end", "2024-01-31");
        params.insert("period_2_start", "2024-02-01");
        params.insert("period_2_end", "2024-02-29");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/comparative",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("period_1_start".to_string(), "2024-01-01".to_string());
                params.insert("period_1_end".to_string(), "2024-01-31".to_string());
                params.insert("period_2_start".to_string(), "2024-02-01".to_string());
                params.insert("period_2_end".to_string(), "2024-02-29".to_string());

                let result = client.reports().comparative(123456, params)?;

                assert_eq!(result.base_period.total_seconds, 576000);
                assert_eq!(result.comparison_period.total_seconds, 604800);
                assert_eq!(result.change.total_seconds_change, 28800);
                assert_eq!(result.change.total_amount_change_percentage, Some(0.05));
                Ok(())
            },
        )
    }

    #[test]
    fn test_export_report() -> Result<()> {
        let response = json!({
            "export_id": "exp_123456",
            "status": "pending",
            "download_url": "https://toggl.com/exports/exp_123456"
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");
        params.insert("export_type", "csv");

        with_mockito_params(
            Method::POST,
            "/reports/api/v3/workspace/123456/export",
            Some(params),
            200,
            Some(response),
            |client| {
                let mut params = BTreeMap::new();
                params.insert("start_date".to_string(), "2024-01-01".to_string());
                params.insert("end_date".to_string(), "2024-01-31".to_string());

                let result = client.reports().export(123456, "csv", params)?;

                assert_eq!(result.export_id, "exp_123456");
                assert_eq!(result.download_url, "https://toggl.com/exports/exp_123456");
                Ok(())
            },
        )
    }
}
