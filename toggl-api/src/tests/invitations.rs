#[cfg(test)]
mod tests {
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;
    use toggl_core::{InvitationId, InvitationItemId, OrganizationId, UserId, WorkspaceId};

    #[test]
    fn test_get_invitation() -> Result<()> {
        let response = json!({
            "id": 12345,
            "email": "user@example.com",
            "invitation_id": "inv_abc123",
            "invite_url": "https://track.toggl.com/invitations/accept/abc123",
            "created_at": "2024-01-01T10:00:00Z",
            "updated_at": "2024-01-01T10:00:00Z",
            "deleted_at": null,
            "sender_id": 98765,
            "sender_name": "Admin User",
            "sender_email": "admin@example.com",
            "recipient_id": null,
            "recipient_email": "user@example.com",
            "recipient_name": null,
            "workspace_id": 111111,
            "workspace_name": "My Workspace",
            "organization_id": 222222,
            "organization_name": "My Organization"
        });

        with_mockito(
            Method::GET,
            "/invitations/abc123",
            200,
            Some(response),
            |client| {
                let invitation = client.invitations().get("abc123")?;

                assert_eq!(invitation.id, InvitationItemId(12345));
                assert_eq!(invitation.email, "user@example.com");
                assert_eq!(invitation.invitation_id, InvitationId::from("inv_abc123"));
                assert_eq!(
                    invitation.invite_url,
                    "https://track.toggl.com/invitations/accept/abc123"
                );
                assert_eq!(invitation.sender_id, UserId(98765));
                assert_eq!(invitation.sender_name, "Admin User");
                assert_eq!(invitation.sender_email, "admin@example.com");
                assert_eq!(invitation.recipient_email, "user@example.com");
                assert_eq!(invitation.workspace_id, WorkspaceId(111111));
                assert_eq!(invitation.workspace_name, "My Workspace");
                assert_eq!(invitation.organization_id, Some(OrganizationId(222222)));
                assert_eq!(
                    invitation.organization_name,
                    Some("My Organization".to_string())
                );
                Ok(())
            },
        )
    }
}
