use super::TogglClient;
use crate::models::api::ids::{SetupId, WorkspaceId};
use crate::models::api::time_entry::TimeEntry;
use crate::models::api::timesheets::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;

pub struct TimesheetsClient {
    client: TogglClient,
}

impl TimesheetsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get user's timesheets
    pub fn get_me_timesheets(&self) -> Result<Vec<Timesheet>> {
        self.client.request(Method::GET, "me/timesheets")
    }

    /// Get timesheet setups for workspace
    pub fn get_timesheet_setups(
        &self,
        workspace_id: WorkspaceId,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<TimesheetSetupsGetPaginatedResponse> {
        let mut params = BTreeMap::new();
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/timesheet_setups", workspace_id),
            &params,
        )
    }

    /// Create timesheet setup
    pub fn create_timesheet_setup(
        &self,
        workspace_id: WorkspaceId,
        payload: &CreateTimesheetSetupPayload,
    ) -> Result<Vec<APITimesheetSetup>> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/timesheet_setups", workspace_id),
            payload,
        )
    }

    /// Update timesheet setup
    pub fn update_timesheet_setup(
        &self,
        workspace_id: WorkspaceId,
        setup_id: SetupId,
        payload: &UpdateTimesheetSetupPayload,
    ) -> Result<APITimesheetSetup> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/timesheet_setups/{}", workspace_id, setup_id),
            payload,
        )
    }

    /// Delete timesheet setup
    pub fn delete_timesheet_setup(
        &self,
        workspace_id: WorkspaceId,
        setup_id: SetupId,
    ) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("workspaces/{}/timesheet_setups/{}", workspace_id, setup_id),
        )
    }

    /// Get workspace timesheets
    pub fn get_timesheets(
        &self,
        workspace_id: WorkspaceId,
        filter: &TimesheetFilter,
    ) -> Result<TimesheetsGetPaginatedResponse> {
        let mut params = BTreeMap::new();
        if let Some(start_date) = filter.start_date {
            params.insert("start_date".to_string(), start_date.to_string());
        }
        if let Some(end_date) = filter.end_date {
            params.insert("end_date".to_string(), end_date.to_string());
        }
        if let Some(member_ids) = filter.member_ids {
            params.insert(
                "member_ids".to_string(),
                member_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(approver_ids) = filter.approver_ids {
            params.insert(
                "approver_ids".to_string(),
                approver_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(timesheet_setup_ids) = filter.timesheet_setup_ids {
            params.insert(
                "timesheet_setup_ids".to_string(),
                timesheet_setup_ids
                    .iter()
                    .map(|id| id.to_string())
                    .collect::<Vec<_>>()
                    .join(","),
            );
        }
        if let Some(statuses) = filter.statuses {
            params.insert("statuses".to_string(), statuses.join(","));
        }
        if let Some(page) = filter.page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = filter.per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = filter.sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = filter.sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("workspaces/{}/timesheets", workspace_id),
            &params,
        )
    }

    /// Update batch of timesheets
    pub fn update_batch_timesheets(
        &self,
        workspace_id: WorkspaceId,
        payloads: &[PutBatchTimesheetPayload],
    ) -> Result<Vec<APITimesheet>> {
        self.client.request_with_body(
            Method::PUT,
            &format!("workspaces/{}/timesheets", workspace_id),
            &payloads,
        )
    }

    /// Get timesheet hours
    pub fn get_timesheet_hours(
        &self,
        workspace_id: WorkspaceId,
        payload: &PostTimesheetHoursPayload,
    ) -> Result<TimesheetHoursResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!("workspaces/{}/timesheets/hours", workspace_id),
            payload,
        )
    }

    /// Update single timesheet
    pub fn update_timesheet(
        &self,
        workspace_id: WorkspaceId,
        setup_id: SetupId,
        start_date: &str,
        payload: &PutTimesheetPayload,
    ) -> Result<APITimesheet> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "workspaces/{}/timesheets/{}/{}",
                workspace_id, setup_id, start_date
            ),
            payload,
        )
    }

    /// Get timesheet time entries
    pub fn get_timesheet_time_entries(
        &self,
        workspace_id: WorkspaceId,
        setup_id: SetupId,
        start_date: &str,
    ) -> Result<Vec<TimeEntry>> {
        self.client.request(
            Method::GET,
            &format!(
                "workspaces/{}/timesheets/{}/{}/time_entries",
                workspace_id, setup_id, start_date
            ),
        )
    }
}
