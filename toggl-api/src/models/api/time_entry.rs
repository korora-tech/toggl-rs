use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{ProjectId, TagId, TaskId, TimeEntryId, UserId, WorkspaceId};

/// Represents a time entry in Toggl.
///
/// Time entries are the core records of tracked time. They can be running (no stop time)
/// or completed. Each entry tracks time spent on a specific task or project.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Get recent time entries
/// let entries = client.get_time_entries(workspace_id, None, None).await?;
///
/// for entry in entries {
///     println!("Description: {:?}", entry.description);
///     println!("Duration: {} seconds", entry.duration);
///     println!("Project: {:?}", entry.project_name);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntry {
    /// Unique identifier for the time entry
    pub id: TimeEntryId,
    /// Workspace this entry belongs to
    pub workspace_id: WorkspaceId,
    /// User who created this entry
    pub user_id: UserId,
    /// Whether this time is billable
    pub billable: bool,
    /// Start time of the entry
    pub start: DateTime<Utc>,
    /// Stop time (None if currently running)
    pub stop: Option<DateTime<Utc>>,
    /// Duration in seconds (negative if currently running)
    pub duration: i64,
    /// Description of the work done
    pub description: Option<String>,
    /// Associated project ID
    pub project_id: Option<ProjectId>,
    /// Associated task ID (if project has tasks)
    pub task_id: Option<TaskId>,
    /// Tag IDs associated with this entry
    pub tag_ids: Option<Vec<TagId>>,
    /// Tag names (legacy field, use tag_ids)
    pub tags: Option<Vec<String>>,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duronly: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared_with: Option<Vec<TimeEntrySharedWith>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expense_ids: Option<Vec<u64>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_color: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_billable: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_avatar_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<WorkspaceId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use project_id instead")]
    pub pid: Option<ProjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use task_id instead")]
    pub tid: Option<TaskId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(note = "Use user_id instead")]
    pub uid: Option<UserId>,
}

/// Information about users with whom a time entry is shared.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntrySharedWith {
    /// Whether the user has accepted the shared entry
    pub accepted: bool,
    /// ID of the user the entry is shared with
    pub user_id: UserId,
    /// Name of the user (for display purposes)
    pub user_name: Option<String>,
}

/// Request structure for creating a new time entry.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::time_entry::CreateTimeEntry;
/// use chrono::Utc;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Create a new time entry
/// let entry = CreateTimeEntry {
///     workspace_id,
///     start: Utc::now(),
///     duration: -1, // Negative duration means it's running
///     description: Some("Working on documentation".to_string()),
///     project_id: Some(ProjectId(789)),
///     task_id: None,
///     tag_ids: Some(vec![TagId(111), TagId(222)]),
///     billable: Some(true),
///     created_with: "toggl-rs".to_string(),
/// };
///
/// let created = client.create_time_entry(workspace_id, entry).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTimeEntry {
    /// Workspace to create the entry in
    pub workspace_id: WorkspaceId,
    /// Start time of the entry
    pub start: DateTime<Utc>,
    /// Duration in seconds (negative for running entries)
    pub duration: i64,
    /// Description of the work
    pub description: Option<String>,
    /// Project to associate with
    pub project_id: Option<ProjectId>,
    /// Task to associate with (requires project_id)
    pub task_id: Option<TaskId>,
    /// Tags to apply
    pub tag_ids: Option<Vec<TagId>>,
    /// Whether the time is billable
    pub billable: Option<bool>,
    /// Application name creating this entry
    pub created_with: String,
}

/// Request structure for updating an existing time entry.
///
/// All fields are optional - only include the fields you want to update.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateTimeEntry {
    pub description: Option<String>,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub tag_ids: Option<Vec<TagId>>,
    pub billable: Option<bool>,
    pub start: Option<DateTime<Utc>>,
    pub stop: Option<DateTime<Utc>>,
    pub duration: Option<i64>,
}

/// Request structure for bulk editing multiple time entries.
///
/// Uses JSON Patch format to apply the same change to multiple entries.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::time_entry::{BulkEditTimeEntries, BulkEditOperation};
/// use serde_json::json;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Add a tag to multiple time entries
/// let bulk_edit = BulkEditTimeEntries {
///     time_entry_ids: vec![TimeEntryId(1), TimeEntryId(2), TimeEntryId(3)],
///     op: BulkEditOperation::Add,
///     path: "/tag_ids".to_string(),
///     value: Some(json!([123])), // Add tag ID 123
/// };
///
/// client.bulk_edit_time_entries(workspace_id, bulk_edit).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkEditTimeEntries {
    /// IDs of time entries to edit
    pub time_entry_ids: Vec<TimeEntryId>,
    /// Operation to perform (add, remove, replace)
    pub op: BulkEditOperation,
    /// JSON Pointer path to the field to modify (e.g., "/tag_ids", "/project_id")
    pub path: String,
    /// Value to apply (required for add and replace operations)
    pub value: Option<serde_json::Value>,
}

/// Operations available for bulk editing time entries.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum BulkEditOperation {
    /// Add values to an array field (e.g., tags)
    Add,
    /// Remove values from an array field
    Remove,
    /// Replace the entire field value
    Replace,
}

/// Validation results for time entries.
///
/// Used to check time entries against workspace constraints and rules.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklist {
    /// Errors that must be fixed before submission
    pub errors: Vec<TimeEntryChecklistError>,
    /// Warnings about potential issues
    pub warnings: Vec<TimeEntryChecklistWarning>,
}

/// Error found during time entry validation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklistError {
    /// Error code for programmatic handling
    pub code: String,
    /// Human-readable error message
    pub message: String,
    /// Time entries affected by this error
    pub time_entry_ids: Vec<TimeEntryId>,
}

/// Warning found during time entry validation.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TimeEntryChecklistWarning {
    /// Warning code for programmatic handling
    pub code: String,
    /// Human-readable warning message
    pub message: String,
    /// Time entries affected by this warning
    pub time_entry_ids: Vec<TimeEntryId>,
}
