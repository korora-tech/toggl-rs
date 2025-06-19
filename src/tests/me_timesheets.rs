#[cfg(test)]
mod tests {
    use crate::{error::Result, tests::with_mockito};
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_get_timesheets() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "approved_or_rejected_at": null,
                    "approved_or_rejected_id": null,
                    "approved_or_rejected_name": null,
                    "approver_avatar_url": null,
                    "approver_id": 456,
                    "approver_name": "John Doe",
                    "approvers": [],
                    "end_date": "2023-01-07",
                    "errors": [],
                    "member_avatar_url": null,
                    "member_id": 789,
                    "member_name": "Jane Smith",
                    "period_editable": true,
                    "period_end": "2023-01-07",
                    "period_locked": false,
                    "period_start": "2023-01-01",
                    "periodicity": "weekly",
                    "permissions": ["edit"],
                    "rejection_comment": null,
                    "reminder_day": null,
                    "reminder_sent_at": null,
                    "reminder_time": null,
                    "reviews": [],
                    "start_date": "2023-01-01",
                    "status": "unsubmitted",
                    "submitted_at": null,
                    "total_duration_in_seconds": 144000,
                    "total_earned_in_cents": null,
                    "user_avatar_url": null,
                    "user_id": 789,
                    "user_name": "Jane Smith",
                    "working_hours_in_minutes": 2400,
                    "workspace_id": 123,
                    "workspace_name": "Test Workspace"
                }
            ],
            "total_count": 1,
            "page": 1,
            "per_page": 50
        });

        with_mockito(
            Method::GET,
            "/me/timesheets",
            200,
            Some(response),
            |client| {
                let result = client.me().get_timesheets(None)?;
                assert!(result.data.is_some());
                let data = result.data.unwrap();
                assert_eq!(data.len(), 1);
                assert_eq!(result.total_count.unwrap(), 1);
                let timesheet = &data[0];
                assert_eq!(timesheet.workspace_id.unwrap(), 123);
                assert_eq!(timesheet.member_id.unwrap(), 789);
                assert_eq!(timesheet.status.as_ref().unwrap(), "unsubmitted");
                Ok(())
            },
        )
    }
}
