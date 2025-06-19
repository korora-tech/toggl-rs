//! # Toggl Rust Client Library
//!
//! This crate provides a unified interface to all Toggl Track APIs.
//!
//! ## Features
//!
//! ### Default Features
//! - `api`: Main Toggl Track API for time tracking, projects, workspaces, etc.
//! - `reports`: Reports API for analytics and time tracking reports
//!
//! ### Optional Features
//! - `webhooks`: Webhooks API for receiving real-time updates
//!
//! ### Feature Groups
//! - `default`: Includes `api` and `reports` (most common use case)
//! - `minimal`: Only the core `api` feature
//! - `analytics`: Same as default (`api` + `reports`)
//! - `full`: All features including webhooks
//!
//! ## Usage Examples
//!
//! ### Basic Usage (default features)
//! ```toml
//! toggl = "0.0.3"
//! ```
//!
//! ### Minimal Usage (only core API)
//! ```toml
//! toggl = { version = "0.0.3", default-features = false, features = ["minimal"] }
//! ```
//!
//! ### Full Usage (all APIs)
//! ```toml
//! toggl = { version = "0.0.3", features = ["full"] }
//! ```
//!
//! ## Example
//!
//! ```rust,no_run
//! use toggl::TogglClient;
//!
//! let client = TogglClient::new("your-api-token".to_string()).unwrap();
//!
//! // Use the client to access various endpoints
//! let user = client.me().get().unwrap();
//! println!("Hello, {}!", user.fullname);
//! ```
//!
//! ## API Availability
//!
//! Different APIs are available based on enabled features:
//!
//! - With `api` feature: `TogglClient` for core functionality
//! - With `reports` feature: `ReportsClient` for analytics
//! - With `webhooks` feature: `WebhooksClient` for real-time updates

#![cfg_attr(docsrs, feature(doc_cfg))]

// Re-export core types
pub use toggl_core::{Auth, Error, Result};

// Re-export API clients based on features
#[cfg(feature = "api")]
#[cfg_attr(docsrs, doc(cfg(feature = "api")))]
pub use toggl_api::{self, TogglClient};

#[cfg(feature = "reports")]
#[cfg_attr(docsrs, doc(cfg(feature = "reports")))]
pub use toggl_reports::{self, ReportsClient};

#[cfg(feature = "webhooks")]
#[cfg_attr(docsrs, doc(cfg(feature = "webhooks")))]
pub use toggl_webhooks::{self, WebhooksClient};

// Re-export models for convenience when api feature is enabled
#[cfg(feature = "api")]
#[cfg_attr(docsrs, doc(cfg(feature = "api")))]
pub use toggl_api::models;

// Provide prelude for common imports
#[cfg(feature = "api")]
#[cfg_attr(docsrs, doc(cfg(feature = "api")))]
pub mod prelude {
    //! Common imports for toggl
    pub use toggl_api::prelude::*;

    #[cfg(feature = "reports")]
    pub use toggl_reports::ReportsClient;

    #[cfg(feature = "webhooks")]
    pub use toggl_webhooks::WebhooksClient;
}
