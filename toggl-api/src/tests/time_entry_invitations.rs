use crate::models::api::ids::{TimeEntryInvitationId, WorkspaceId};
use crate::tests::with_mockito;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

#[test]
fn test_get_time_entry_invitations() -> Result<()> {
    let response = json!([
        {
            "time_entry_invitation_id": 1,
            "workspace_id": 987654,
            "shared_by_user_id": 123456,
            "shared_by_user_name": "John Doe",
            "time_entry": {
                "id": 999,
                "description": "Code review",
                "duration": 3600,
                "start": "2024-01-15T10:00:00Z",
                "stop": "2024-01-15T11:00:00Z",
                "workspace_id": 987654,
                "user_id": 123456,
                "project_id": 555,
                "billable": true,
                "at": "2024-01-15T10:00:00Z"
            }
        },
        {
            "time_entry_invitation_id": 2,
            "workspace_id": 987654,
            "shared_by_user_id": 789012,
            "shared_by_user_name": "Jane Smith",
            "time_entry": {
                "id": 1000,
                "description": "Team meeting",
                "duration": 1800,
                "start": "2024-01-15T14:00:00Z",
                "stop": "2024-01-15T14:30:00Z",
                "workspace_id": 987654,
                "user_id": 789012,
                "project_id": 555,
                "billable": false,
                "at": "2024-01-15T14:00:00Z"
            }
        }
    ]);

    with_mockito(
        Method::GET,
        "/workspaces/987654/time_entry_invitations",
        200,
        Some(response),
        |client| {
            let invitations = client
                .time_entry_invitations()
                .get_invitations(WorkspaceId(987654))?;
            assert_eq!(invitations.len(), 2);
            assert_eq!(
                invitations[0].time_entry_invitation_id,
                TimeEntryInvitationId(1)
            );
            assert_eq!(invitations[0].shared_by_user_name, "John Doe");
            assert_eq!(
                invitations[1].time_entry_invitation_id,
                TimeEntryInvitationId(2)
            );
            assert_eq!(invitations[1].shared_by_user_name, "Jane Smith");
            Ok(())
        },
    )
}

#[test]
fn test_accept_time_entry_invitation() -> Result<()> {
    with_mockito(
        Method::POST,
        "/workspaces/987654/time_entry_invitations/3/accept",
        204,
        None,
        |client| {
            client
                .time_entry_invitations()
                .accept_invitation(WorkspaceId(987654), TimeEntryInvitationId(3))?;
            Ok(())
        },
    )
}

#[test]
fn test_reject_time_entry_invitation() -> Result<()> {
    with_mockito(
        Method::POST,
        "/workspaces/987654/time_entry_invitations/4/reject",
        204,
        None,
        |client| {
            client
                .time_entry_invitations()
                .reject_invitation(WorkspaceId(987654), TimeEntryInvitationId(4))?;
            Ok(())
        },
    )
}
