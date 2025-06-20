use serde::{Deserialize, Serialize};
use toggl_core::{BookmarkId, CreatorId, GroupId, ReportId, UserId, WorkspaceId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduledReport {
    pub bookmark_id: Option<BookmarkId>,
    pub created_at: Option<String>,
    pub creator_id: Option<CreatorId>,
    pub deleted_at: Option<String>,
    pub frequency: Option<i32>,
    pub group_ids: Option<Vec<GroupId>>,
    pub report_id: Option<ReportId>,
    pub user_ids: Option<Vec<UserId>>,
    pub workspace_id: Option<WorkspaceId>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateScheduledReportPayload {
    pub bookmark_id: BookmarkId,
    pub frequency: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<GroupId>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<UserId>>,
}
