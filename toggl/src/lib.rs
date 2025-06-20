//! # Toggl Rust Client Library
//!
//! This crate provides a unified interface to all Toggl Track APIs.
//!
//! ## Features
//!
//! By default, only the model types are available. You can enable client functionality
//! with the following features:
//!
//! - `api-client`: Main Toggl Track API client for time tracking, projects, workspaces, etc.
//! - `reports-client`: Reports API client for analytics and time tracking reports
//! - `webhooks-client`: Webhooks API client for receiving real-time updates
//!
//! ## Usage Examples
//!
//! ### Models Only (default)
//! ```toml
//! toggl = "0.0.3"
//! ```
//!
//! ### With API Client
//! ```toml
//! toggl = { version = "0.0.3", features = ["api-client"] }
//! ```
//!
//! ### With Reports Client
//! ```toml
//! toggl = { version = "0.0.3", features = ["reports-client"] }
//! ```
//!
//! ### With All Clients
//! ```toml
//! toggl = { version = "0.0.3", features = ["api-client", "reports-client", "webhooks-client"] }
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! # #[cfg(feature = "api-client")]
//! # {
//! use toggl::TogglClient;
//!
//! let client = TogglClient::new("your-api-token").unwrap();
//!
//! // Use the client to access various endpoints
//! # async {
//! let user = client.get_me().await.unwrap();
//! println!("Hello, {}!", user.fullname);
//! # };
//! # }
//! ```
//!
//! ## API Availability
//!
//! Different APIs are available based on enabled features:
//!
//! - Models are always available for all APIs
//! - With `api-client` feature: `TogglClient` for core functionality
//! - With `reports-client` feature: `ReportsClient` for analytics
//! - With `webhooks-client` feature: `WebhooksClient` for real-time updates

#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export core types
pub use toggl_core::{Error, Result};

// Re-export models from all crates (always available)
pub use toggl_api::models as api_models;
pub use toggl_reports::models as reports_models;
pub use toggl_webhooks::events as webhooks_events;
pub use toggl_webhooks::models as webhooks_models;

// Re-export API clients based on features
#[cfg(feature = "api-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "api-client")))]
pub use toggl_api::{Auth, TogglClient};

#[cfg(feature = "reports-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "reports-client")))]
pub use toggl_reports::ReportsClient;

#[cfg(feature = "webhooks-client")]
#[cfg_attr(docsrs, doc(cfg(feature = "webhooks-client")))]
pub use toggl_webhooks::WebhooksClient;

// Provide prelude for common imports
pub mod prelude {
    //! Common imports for toggl

    // Always available - core types
    pub use toggl_core::{Error, Result};

    // Always available - common API models
    pub use crate::api_models::{
        Client, ClientId, CreateClient, CreateProject, CreateTag, CreateTimeEntry, CreateWorkspace,
        Project, ProjectId, TagId, TimeEntry, TimeEntryId, UpdateClient, UpdateProject, UpdateTag,
        UpdateTimeEntry, UpdateWorkspace, User, UserId, Workspace, WorkspaceId,
    };
    // Tag has naming conflicts, so be explicit
    pub use crate::api_models::api::tag::Tag as ApiTag;

    // Always available - webhook models
    pub use toggl_webhooks::{
        AppHealth, CreateSubscription, EventMetadata, EventPayload, Subscription,
        UpdateSubscription, WebhookEvent,
    };

    // Conditionally available - clients
    #[cfg(feature = "api-client")]
    pub use toggl_api::{Auth, TogglClient};

    #[cfg(feature = "reports-client")]
    pub use toggl_reports::ReportsClient;

    #[cfg(feature = "webhooks-client")]
    pub use toggl_webhooks::WebhooksClient;
}
