use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, WorkspaceId};

/// Represents a client in Toggl.
///
/// Clients are used to organize projects and time tracking for different customers
/// or departments. Projects can be associated with clients for better organization
/// and reporting.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// // Get all clients
/// let clients = client.get_clients(workspace_id).await?;
///
/// for c in clients {
///     println!("Client: {} (ID: {})", c.name, c.id);
///     println!("Archived: {}", c.archived);
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Client {
    /// Unique identifier for the client
    pub id: ClientId,
    /// Workspace this client belongs to
    pub workspace_id: WorkspaceId,
    /// Client name
    pub name: String,
    /// Whether this client is archived
    pub archived: bool,
    /// Server timestamp when this data was retrieved
    pub at: DateTime<Utc>,
    /// When the client was deleted (if applicable)
    pub server_deleted_at: Option<DateTime<Utc>>,
    /// User's permissions for this client
    pub permissions: Option<Vec<String>>,

    #[deprecated(note = "Use workspace_id instead")]
    pub wid: Option<u64>,
}

/// Request structure for creating a new client.
///
/// # Example
/// ```no_run
/// use toggl_api::prelude::*;
/// use toggl_api::models::api::client::CreateClient;
///
/// # async fn example() -> Result<()> {
/// let client = TogglClient::new("your-api-token");
/// let workspace_id = WorkspaceId(123456);
///
/// let new_client = CreateClient {
///     workspace_id,
///     name: "Acme Corporation".to_string(),
/// };
///
/// let created = client.create_client(workspace_id, new_client).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateClient {
    pub workspace_id: WorkspaceId,
    pub name: String,
}

/// Request structure for updating an existing client.
///
/// All fields are optional - only include the fields you want to update.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct UpdateClient {
    pub name: Option<String>,
    pub archived: Option<bool>,
}

/// Request structure for archiving multiple clients at once.
///
/// Archiving clients also archives all their associated projects.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct BulkArchiveClients {
    pub client_ids: Vec<ClientId>,
}

/// Response from bulk archive operation.
///
/// Contains IDs of all affected clients and projects.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ArchiveClientsResponse {
    /// IDs of clients that were archived
    pub client_ids: Option<Vec<ClientId>>,
    /// IDs of projects that were archived as a result
    pub project_ids: Option<Vec<u64>>,
}
