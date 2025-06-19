//! # Toggl API Client
//!
//! Client library for the main Toggl Track API.
//!
//! This crate provides access to all core Toggl functionality including:
//! - Time tracking
//! - Projects and tasks
//! - Workspaces and organizations
//! - Users and teams
//! - Tags and clients

pub mod client;
pub mod models;
pub mod prelude;

#[cfg(test)]
mod tests;

pub use client::TogglClient;
pub use toggl_core::{Auth, Error, Result};

// Re-export commonly used types
pub use models::*;
