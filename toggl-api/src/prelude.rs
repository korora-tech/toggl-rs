//! Common imports for toggl-api
//!
//! This module provides commonly used types and traits that are frequently
//! needed when working with the Toggl API.

// Core types
pub use crate::client::TogglClient;
pub use toggl_core::{Error, Result};

// Common ID types from toggl_core
pub use toggl_core::{
    ClientId, GroupId, OrganizationId, ProjectId, TagId, TaskId, TimeEntryId, UserId, WorkspaceId,
};

// User and workspace basics
pub use crate::models::api::user::User;
pub use crate::models::api::workspace::{
    CreateWorkspace, UpdateWorkspace, Workspace, WorkspaceUser,
};

// Projects and time tracking
pub use crate::models::api::project::{CreateProject, Project, UpdateProject};
pub use crate::models::api::task::Task;
pub use crate::models::api::time_entry::{CreateTimeEntry, TimeEntry, UpdateTimeEntry};

// Common traits and utilities
pub use chrono::{DateTime, NaiveDate, Utc};
pub use serde::{Deserialize, Serialize};
