use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use toggl_core::{TagId, WorkspaceId};

/// Represents a tag in Toggl.
///
/// Tags are used to categorize and organize time entries across projects.
/// They provide a flexible way to add metadata to time tracking that can
/// be used for reporting and filtering.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Get all tags
/// let tags = client.get_tags(workspace_id).await?;
///
/// for tag in tags {
///     println!("Tag: {} (ID: {})", tag.name, tag.id);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    /// Unique identifier for the tag
    pub id: TagId,
    /// Workspace this tag belongs to
    pub workspace_id: WorkspaceId,
    /// Tag name
    pub name: String,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// When the tag was deleted (if applicable)
    pub deleted_at: Option<DateTime<Utc>>,
    /// User's permissions for this tag
    pub permissions: Option<Vec<String>>,
}

/// Request structure for creating a new tag.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::tag::CreateTag;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// let new_tag = CreateTag {
///     workspace_id,
///     name: "urgent".to_string(),
/// };
///
/// let tag = client.create_tag(workspace_id, new_tag).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateTag {
    pub workspace_id: WorkspaceId,
    pub name: String,
}

/// Request structure for updating an existing tag.
///
/// Currently only the tag name can be updated.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateTag {
    pub name: String,
}
