#[cfg(test)]
mod tests {
    use crate::models::api::timesheets::*;
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn test_get_me_timesheets() -> Result<()> {
        let response = json!([
            {
                "timesheet_id": 1,
                "workspace_id": 123,
                "timesheet_setup_id": 456,
                "start_date": "2024-01-01",
                "status": "pending",
                "submitted_at": null,
                "approved": false,
                "timezone": "UTC",
                "working_hours_in_minutes": 480
            }
        ]);

        with_mockito(
            Method::GET,
            "/me/timesheets",
            200,
            Some(response),
            |client| {
                let timesheets = client.timesheets().get_me_timesheets()?;
                assert_eq!(timesheets.len(), 1);
                assert_eq!(timesheets[0].timesheet_id, Some(1));
                assert_eq!(timesheets[0].workspace_id, Some(123));
                assert_eq!(timesheets[0].status, Some("pending".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_timesheet_setups() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 1,
                    "workspace_id": 123,
                    "member_id": 456,
                    "member_name": "John Doe",
                    "approver_id": 789,
                    "approver_name": "Jane Smith",
                    "start_date": "2024-01-01",
                    "periodicity": "weekly",
                    "reminder_day": "monday",
                    "reminder_time": "09:00"
                }
            ]
        });

        with_mockito(
            Method::GET,
            "/workspaces/123/timesheet_setups",
            200,
            Some(response),
            |client| {
                let result = client
                    .timesheets()
                    .get_timesheet_setups(123, None, None, None, None)?;
                assert_eq!(result.data.as_ref().unwrap().len(), 1);
                let setup = &result.data.as_ref().unwrap()[0];
                assert_eq!(setup.id, Some(1));
                assert_eq!(setup.workspace_id, Some(123));
                assert_eq!(setup.member_name, Some("John Doe".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_timesheet_setup() -> Result<()> {
        let payload = CreateTimesheetSetupPayload {
            approver_id: Some(789),
            approver_ids: None,
            member_ids: Some(vec![456]),
            periodicity: Some("weekly".to_string()),
            reminder_day: Some(Weekday::Monday),
            reminder_time: Some("09:00".to_string()),
            required_approvers: Some(1),
            start_date: Some("2024-01-01".to_string()),
        };

        let response = json!([
            {
                "id": 1,
                "workspace_id": 123,
                "member_id": 456,
                "member_name": "John Doe",
                "approver_id": 789,
                "approver_name": "Jane Smith",
                "start_date": "2024-01-01",
                "periodicity": "weekly"
            }
        ]);

        with_mockito(
            Method::POST,
            "/workspaces/123/timesheet_setups",
            200,
            Some(response),
            |client| {
                let setups = client.timesheets().create_timesheet_setup(123, &payload)?;
                assert_eq!(setups.len(), 1);
                assert_eq!(setups[0].id, Some(1));
                assert_eq!(setups[0].member_id, Some(456));
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_timesheet_setup() -> Result<()> {
        let payload = UpdateTimesheetSetupPayload {
            approver_id: Some(999),
            approver_ids: None,
            end_date: Some("2024-12-31".to_string()),
            reminder_day: Some(Weekday::Friday),
            reminder_time: Some("17:00".to_string()),
            required_approvers: Some(2),
        };

        let response = json!({
            "id": 1,
            "workspace_id": 123,
            "member_id": 456,
            "approver_id": 999,
            "end_date": "2024-12-31",
            "reminder_day": "friday",
            "reminder_time": "17:00"
        });

        with_mockito(
            Method::PUT,
            "/workspaces/123/timesheet_setups/1",
            200,
            Some(response),
            |client| {
                let setup = client
                    .timesheets()
                    .update_timesheet_setup(123, 1, &payload)?;
                assert_eq!(setup.id, Some(1));
                assert_eq!(setup.approver_id, Some(999));
                assert_eq!(setup.end_date, Some("2024-12-31".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_timesheet_setup() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/workspaces/123/timesheet_setups/1",
            204,
            None,
            |client| {
                client.timesheets().delete_timesheet_setup(123, 1)?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_timesheets() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "timesheet_setup_id": 1,
                    "workspace_id": 123,
                    "member_id": 456,
                    "member_name": "John Doe",
                    "start_date": "2024-01-01",
                    "end_date": "2024-01-07",
                    "status": "submitted",
                    "submitted_at": "2024-01-08T10:00:00Z",
                    "working_hours_in_minutes": 2400
                }
            ],
            "page": 1,
            "per_page": 50,
            "total_count": 1
        });

        let mut params = BTreeMap::new();
        params.insert("start_date", "2024-01-01");
        params.insert("end_date", "2024-01-31");

        with_mockito_params(
            Method::GET,
            "/workspaces/123/timesheets",
            Some(params),
            200,
            Some(response),
            |client| {
                let filter = TimesheetFilter {
                    start_date: Some("2024-01-01"),
                    end_date: Some("2024-01-31"),
                    ..Default::default()
                };
                let result = client.timesheets().get_timesheets(123, &filter)?;
                assert_eq!(result.page, Some(1));
                assert_eq!(result.total_count, Some(1));
                assert_eq!(result.data.as_ref().unwrap().len(), 1);
                let timesheet = &result.data.as_ref().unwrap()[0];
                assert_eq!(timesheet.status, Some("submitted".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_batch_timesheets() -> Result<()> {
        let payloads = vec![
            PutBatchTimesheetPayload {
                timesheet_setup_id: Some(1),
                start_date: Some("2024-01-01".to_string()),
                status: Some("approved".to_string()),
                force_approved: Some(false),
                rejection_comment: None,
            },
            PutBatchTimesheetPayload {
                timesheet_setup_id: Some(2),
                start_date: Some("2024-01-08".to_string()),
                status: Some("rejected".to_string()),
                force_approved: Some(false),
                rejection_comment: Some("Missing hours".to_string()),
            },
        ];

        let response = json!([
            {
                "timesheet_setup_id": 1,
                "start_date": "2024-01-01",
                "status": "approved",
                "approved_or_rejected_at": "2024-01-15T10:00:00Z"
            },
            {
                "timesheet_setup_id": 2,
                "start_date": "2024-01-08",
                "status": "rejected",
                "rejection_comment": "Missing hours"
            }
        ]);

        with_mockito(
            Method::PUT,
            "/workspaces/123/timesheets",
            200,
            Some(response),
            |client| {
                let timesheets = client
                    .timesheets()
                    .update_batch_timesheets(123, &payloads)?;
                assert_eq!(timesheets.len(), 2);
                assert_eq!(timesheets[0].status, Some("approved".to_string()));
                assert_eq!(timesheets[1].status, Some("rejected".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_timesheet_hours() -> Result<()> {
        let payload = PostTimesheetHoursPayload {
            timesheet_setup_id: Some(1),
            start_date: Some("2024-01-01".to_string()),
        };

        let response = json!({
            "timesheet_setup_id": 1,
            "start_date": "2024-01-01",
            "total_seconds": 144000,
            "working_hours_in_minutes": 2400
        });

        with_mockito(
            Method::POST,
            "/workspaces/123/timesheets/hours",
            200,
            Some(response),
            |client| {
                let hours = client.timesheets().get_timesheet_hours(123, &payload)?;
                assert_eq!(hours.timesheet_setup_id, Some(1));
                assert_eq!(hours.total_seconds, Some(144000));
                assert_eq!(hours.working_hours_in_minutes, Some(2400));
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_timesheet() -> Result<()> {
        let payload = PutTimesheetPayload {
            status: Some("submitted".to_string()),
            force_approved: Some(false),
            rejection_comment: None,
        };

        let response = json!({
            "timesheet_setup_id": 1,
            "start_date": "2024-01-01",
            "status": "submitted",
            "submitted_at": "2024-01-08T10:00:00Z",
            "working_hours_in_minutes": 2400
        });

        with_mockito(
            Method::PUT,
            "/workspaces/123/timesheets/1/2024-01-01",
            200,
            Some(response),
            |client| {
                let timesheet =
                    client
                        .timesheets()
                        .update_timesheet(123, 1, "2024-01-01", &payload)?;
                assert_eq!(timesheet.status, Some("submitted".to_string()));
                assert_eq!(timesheet.timesheet_setup_id, Some(1));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_timesheet_time_entries() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "description": "Working on project",
                "start": "2024-01-01T09:00:00Z",
                "stop": "2024-01-01T17:00:00Z",
                "duration": 28800,
                "workspace_id": 123,
                "user_id": 789,
                "project_id": 456,
                "task_id": null,
                "billable": true,
                "at": "2024-01-01T09:00:00Z"
            }
        ]);

        with_mockito(
            Method::GET,
            "/workspaces/123/timesheets/1/2024-01-01/time_entries",
            200,
            Some(response),
            |client| {
                let entries =
                    client
                        .timesheets()
                        .get_timesheet_time_entries(123, 1, "2024-01-01")?;
                assert_eq!(entries.len(), 1);
                assert_eq!(entries[0].id, 1);
                assert_eq!(entries[0].duration, 28800);
                Ok(())
            },
        )
    }
}
