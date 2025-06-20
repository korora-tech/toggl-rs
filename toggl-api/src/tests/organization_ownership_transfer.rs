use crate::models::api::organization::CreateOwnershipTransfer;
use crate::tests::*;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;
use toggl_core::{OrganizationId, TransferId, UserId};

#[test]
fn test_accept_ownership_transfer() -> Result<()> {
    let organization_id = OrganizationId(12345);
    let transfer_id = TransferId(67890);

    with_mockito(
        Method::POST,
        &format!(
            "/organizations/{}/owner/transfer/{}/accept",
            organization_id, transfer_id
        ),
        200,
        None,
        |client| {
            client
                .organizations()
                .accept_ownership_transfer(organization_id, transfer_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_reject_ownership_transfer() -> Result<()> {
    let organization_id = OrganizationId(12345);
    let transfer_id = TransferId(67890);

    with_mockito(
        Method::POST,
        &format!(
            "/organizations/{}/owner/transfer/{}/reject",
            organization_id, transfer_id
        ),
        200,
        None,
        |client| {
            client
                .organizations()
                .reject_ownership_transfer(organization_id, transfer_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_cancel_ownership_transfer() -> Result<()> {
    let organization_id = OrganizationId(12345);
    let transfer_id = TransferId(67890);

    with_mockito(
        Method::POST,
        &format!(
            "/organizations/{}/owner/transfer/{}/cancel",
            organization_id, transfer_id
        ),
        200,
        None,
        |client| {
            client
                .organizations()
                .cancel_ownership_transfer(organization_id, transfer_id)?;
            Ok(())
        },
    )
}

#[test]
fn test_ownership_transfer_workflow() -> Result<()> {
    let organization_id = OrganizationId(12345);
    let new_owner_id = UserId(99999);
    let transfer_id = TransferId(67890);

    // First create a transfer
    let create_request = CreateOwnershipTransfer { new_owner_id };

    let create_response = json!({
        "id": transfer_id,
        "organization_id": organization_id,
        "current_owner_id": 11111,
        "new_owner_id": new_owner_id,
        "status": "pending",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": null
    });

    with_mockito(
        Method::POST,
        &format!("/organizations/{}/owner/transfer", organization_id),
        200,
        Some(create_response),
        |client| {
            let transfer = client
                .organizations()
                .create_ownership_transfer(organization_id, &create_request)?;
            assert_eq!(transfer.id, transfer_id);
            assert_eq!(transfer.new_owner_id, new_owner_id);
            assert_eq!(transfer.status, "pending");
            Ok(())
        },
    )?;

    // Then check the status
    let status_response = json!({
        "id": transfer_id,
        "organization_id": organization_id,
        "current_owner_id": 11111,
        "new_owner_id": new_owner_id,
        "status": "pending",
        "created_at": "2023-01-01T00:00:00Z",
        "updated_at": null
    });

    with_mockito(
        Method::GET,
        &format!(
            "/organizations/{}/owner/transfer/{}",
            organization_id, transfer_id
        ),
        200,
        Some(status_response),
        |client| {
            let transfer = client
                .organizations()
                .get_ownership_transfer(organization_id, transfer_id)?;
            assert_eq!(transfer.id, transfer_id);
            assert_eq!(transfer.status, "pending");
            Ok(())
        },
    )
}
