use crate::ids::*;

pub const TEST_API_TOKEN: &str = "test_api_token";

// Common test IDs
pub const TEST_WORKSPACE_ID: u64 = 1234567;
pub const TEST_PROJECT_ID: u64 = 123456789;
pub const TEST_USER_ID: u64 = 87654321;
pub const TEST_CLIENT_ID: u64 = 11111111;
pub const TEST_ORGANIZATION_ID: u64 = 7654321;
pub const TEST_TASK_ID: u64 = 99999999;
pub const TEST_TAG_ID: u64 = 88888888;
pub const TEST_TIME_ENTRY_ID: u64 = 77777777;

// Helper functions to create ID types
pub fn test_workspace_id() -> WorkspaceId {
    WorkspaceId(TEST_WORKSPACE_ID)
}

pub fn test_project_id() -> ProjectId {
    ProjectId(TEST_PROJECT_ID)
}

pub fn test_user_id() -> UserId {
    UserId(TEST_USER_ID)
}

pub fn test_client_id() -> ClientId {
    ClientId(TEST_CLIENT_ID)
}

pub fn test_organization_id() -> OrganizationId {
    OrganizationId(TEST_ORGANIZATION_ID)
}

pub fn test_task_id() -> TaskId {
    TaskId(TEST_TASK_ID)
}

pub fn test_tag_id() -> TagId {
    TagId(TEST_TAG_ID)
}

pub fn test_time_entry_id() -> TimeEntryId {
    TimeEntryId(TEST_TIME_ENTRY_ID)
}
