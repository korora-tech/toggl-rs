use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub approved: Option<bool>,
    pub approved_or_rejected_at: Option<String>,
    pub approved_or_rejected_id: Option<i64>,
    pub created_at: Option<String>,
    pub deleted_at: Option<String>,
    pub force_approved: Option<bool>,
    pub rejection_comment: Option<String>,
    pub reminder_sent_at: Option<String>,
    pub required_approvers: Option<i32>,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub submitted_at: Option<String>,
    pub timesheet_id: Option<i64>,
    pub timesheet_setup_id: Option<i64>,
    pub timezone: Option<String>,
    pub updated_at: Option<String>,
    pub working_hours_in_minutes: Option<i32>,
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APITimesheet {
    pub approved_or_rejected_at: Option<String>,
    pub approved_or_rejected_id: Option<i64>,
    pub approved_or_rejected_name: Option<String>,
    pub approver_avatar_url: Option<String>,
    pub approver_id: Option<i64>,
    pub approver_name: Option<String>,
    pub approvers: Option<Vec<TimesheetApprover>>,
    pub end_date: Option<String>,
    pub errors: Option<Vec<TimesheetError>>,
    pub member_avatar_url: Option<String>,
    pub member_id: Option<i64>,
    pub member_name: Option<String>,
    pub period_editable: Option<bool>,
    pub period_end: Option<String>,
    pub period_locked: Option<bool>,
    pub period_start: Option<String>,
    pub periodicity: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub rejection_comment: Option<String>,
    pub reminder_day: Option<Weekday>,
    pub reminder_sent_at: Option<String>,
    pub reminder_time: Option<String>,
    pub reviews: Option<Vec<Review>>,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub submitted_at: Option<String>,
    pub timesheet_setup_id: Option<i64>,
    pub timezone: Option<String>,
    pub working_hours_in_minutes: Option<i32>,
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APITimesheetSetup {
    pub approver_avatar_url: Option<String>,
    pub approver_id: Option<i64>,
    pub approver_name: Option<String>,
    pub approvers: Option<Vec<TimesheetSetupApprovers>>,
    pub end_date: Option<String>,
    pub errors: Option<Vec<TimesheetSetupError>>,
    pub id: Option<i64>,
    pub member_avatar_url: Option<String>,
    pub member_id: Option<i64>,
    pub member_name: Option<String>,
    pub periodicity: Option<String>,
    pub permissions: Option<Vec<String>>,
    pub reminder_day: Option<Weekday>,
    pub reminder_time: Option<String>,
    pub required_approvers: Option<i32>,
    pub start_date: Option<String>,
    pub workspace_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTimesheetSetupPayload {
    pub approver_id: Option<i64>,
    pub approver_ids: Option<Vec<i64>>,
    pub member_ids: Option<Vec<i64>>,
    pub periodicity: Option<String>,
    pub reminder_day: Option<Weekday>,
    pub reminder_time: Option<String>,
    pub required_approvers: Option<i32>,
    pub start_date: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTimesheetSetupPayload {
    pub approver_id: Option<i64>,
    pub approver_ids: Option<Vec<i64>>,
    pub end_date: Option<String>,
    pub reminder_day: Option<Weekday>,
    pub reminder_time: Option<String>,
    pub required_approvers: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutBatchTimesheetPayload {
    pub force_approved: Option<bool>,
    pub rejection_comment: Option<String>,
    pub start_date: Option<String>,
    pub status: Option<String>,
    pub timesheet_setup_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutTimesheetPayload {
    pub force_approved: Option<bool>,
    pub rejection_comment: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTimesheetHoursPayload {
    pub start_date: Option<String>,
    pub timesheet_setup_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetHoursResponse {
    pub start_date: Option<String>,
    pub timesheet_setup_id: Option<i64>,
    pub total_seconds: Option<i64>,
    pub working_hours_in_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetSetupsGetPaginatedResponse {
    pub data: Option<Vec<APITimesheetSetup>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetsGetPaginatedResponse {
    pub data: Option<Vec<APITimesheet>>,
    pub page: Option<i32>,
    pub per_page: Option<i32>,
    pub total_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetApprover {
    pub active: Option<bool>,
    pub avatar_url: Option<String>,
    pub deleted: Option<bool>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetSetupApprovers {
    pub active: Option<bool>,
    pub avatar_url: Option<String>,
    pub deleted: Option<bool>,
    pub name: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetError {
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimesheetSetupError {
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub approved: Option<bool>,
    pub avatar_url: Option<String>,
    pub force_approved: Option<bool>,
    pub name: Option<String>,
    pub rejection_comment: Option<String>,
    pub updated_at: Option<String>,
    pub user_id: Option<i64>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Weekday {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}

#[derive(Debug, Clone, Default)]
pub struct TimesheetFilter<'a> {
    pub start_date: Option<&'a str>,
    pub end_date: Option<&'a str>,
    pub member_ids: Option<&'a [u64]>,
    pub approver_ids: Option<&'a [u64]>,
    pub timesheet_setup_ids: Option<&'a [u64]>,
    pub statuses: Option<&'a [&'a str]>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
    pub sort_field: Option<&'a str>,
    pub sort_order: Option<&'a str>,
}
