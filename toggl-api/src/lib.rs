//! # Toggl API Client
//!
//! A comprehensive Rust client library for the [Toggl Track API v9](https://developers.track.toggl.com/docs/).
//!
//! This crate provides type-safe access to all Toggl Track functionality including:
//! - Time tracking and time entries
//! - Projects, tasks, and clients  
//! - Workspaces and organizations
//! - Users and team management
//! - Tags and categorization
//! - Reports and analytics
//! - Authentication and SSO
//!
//! ## Quick Start
//!
//! ```no_run
//! use toggl_api::prelude::*;
//!
//! # async fn example() -> Result<()> {
//! // Create a client with your API token
//! let client = TogglClient::new("your-api-token");
//!
//! // Get current user
//! let me = client.get_me().await?;
//! println!("Hello, {}!", me.fullname);
//!
//! // List workspaces
//! let workspaces = client.get_workspaces().await?;
//!
//! // Create a time entry
//! let workspace_id = workspaces[0].id;
//! let entry = CreateTimeEntry {
//!     workspace_id,
//!     description: Some("Working on toggl-rs".to_string()),
//!     start: chrono::Utc::now(),
//!     duration: -1, // Negative means it's running
//!     created_with: "toggl-rs".to_string(),
//!     ..Default::default()
//! };
//!
//! let time_entry = client.create_time_entry(workspace_id, entry).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Features
//!
//! This crate supports the following feature flags:
//!
//! - `default`: Core API functionality
//! - `reports`: Reporting API support
//! - `minimal`: Just core API without extras
//! - `analytics`: API + reports  
//! - `full`: All features enabled
//!
//! ## Authentication
//!
//! The client uses HTTP Basic Authentication with your API token as the password:
//!
//! ```no_run
//! # use toggl_api::TogglClient;
//! # use toggl_api::Result;
//! # fn example() -> Result<()> {
//! let client = TogglClient::new("your-api-token")?;
//! # Ok(())
//! # }
//! ```
//!
//! You can find your API token in your [Toggl Track profile settings](https://track.toggl.com/profile).
//!
//! ## Error Handling
//!
//! All API methods return `Result<T>` where errors are of type `toggl_core::Error`.
//! Common error types include:
//!
//! - `Error::Unauthorized` - Invalid API token
//! - `Error::NotFound` - Resource not found
//! - `Error::RateLimited` - API rate limit exceeded
//! - `Error::Network` - Network connectivity issues

#[cfg(feature = "client")]
pub mod client;
pub mod models;
pub mod prelude;

#[cfg(all(test, feature = "client"))]
mod tests;

#[cfg(feature = "client")]
pub use client::TogglClient;
#[cfg(feature = "client")]
pub use toggl_core::Auth;
pub use toggl_core::{Error, Result};

// Re-export commonly used types
pub use models::*;
