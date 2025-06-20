use super::TogglClient;
use crate::models::api::invitation::*;
use crate::models::api::organization::*;
use crate::models::api::organization_subscription::*;
use crate::models::api::segmentation::*;
use reqwest::Method;
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{GroupId, OrganizationId, PricingPlanId, TransferId, UserId, WorkspaceId};

pub struct OrganizationsClient {
    client: TogglClient,
}

impl OrganizationsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }

    /// Get organization
    pub fn get(&self, organization_id: OrganizationId) -> Result<Organization> {
        self.client
            .request(Method::GET, &format!("organizations/{}", organization_id))
    }

    /// Update organization
    pub fn update(
        &self,
        organization_id: OrganizationId,
        org_update: &UpdateOrganization,
    ) -> Result<Organization> {
        self.client.request_with_body(
            Method::PUT,
            &format!("organizations/{}", organization_id),
            org_update,
        )
    }

    /// Get organization users
    pub fn get_users(
        &self,
        organization_id: OrganizationId,
        filter: Option<&str>,
        page: Option<u32>,
    ) -> Result<Vec<OrganizationUser>> {
        let mut params = BTreeMap::new();
        if let Some(filter) = filter {
            params.insert("filter".to_string(), filter.to_string());
        }
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("organizations/{}/users", organization_id),
            &params,
        )
    }

    /// Get organization workspaces
    pub fn get_workspaces(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<OrganizationWorkspace>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/workspaces", organization_id),
        )
    }

    /// Get organization groups
    pub fn get_groups(&self, organization_id: OrganizationId) -> Result<Vec<OrganizationGroup>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/groups", organization_id),
        )
    }

    /// Create organization group
    pub fn create_group(
        &self,
        organization_id: OrganizationId,
        group: &CreateOrganizationGroup,
    ) -> Result<OrganizationGroup> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/groups", organization_id),
            group,
        )
    }

    /// Update organization group
    pub fn update_group(
        &self,
        organization_id: OrganizationId,
        group_id: GroupId,
        group: &UpdateOrganizationGroup,
    ) -> Result<OrganizationGroup> {
        self.client.request_with_body(
            Method::PUT,
            &format!("organizations/{}/groups/{}", organization_id, group_id),
            group,
        )
    }

    /// Update organization group with patch
    pub fn patch_group(
        &self,
        organization_id: OrganizationId,
        group_id: GroupId,
        group: &UpdateOrganizationGroup,
    ) -> Result<OrganizationGroup> {
        self.client.request_with_body(
            Method::PATCH,
            &format!("organizations/{}/groups/{}", organization_id, group_id),
            group,
        )
    }

    /// Delete organization group
    pub fn delete_group(&self, organization_id: OrganizationId, group_id: GroupId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("organizations/{}/groups/{}", organization_id, group_id),
        )
    }

    /// Get organization owner
    pub fn get_owner(&self, organization_id: OrganizationId) -> Result<OrganizationOwner> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/owner", organization_id),
        )
    }

    /// Create organization
    pub fn create(&self, org_create: &CreateOrganization) -> Result<Organization> {
        self.client
            .request_with_body(Method::POST, "organizations", org_create)
    }

    /// Get organization invitations
    pub fn get_invitations(&self, organization_id: OrganizationId) -> Result<Vec<Invitation>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/invitations", organization_id),
        )
    }

    /// Create organization invitation
    pub fn create_invitation(
        &self,
        organization_id: OrganizationId,
        invitation: &CreateInvitation,
    ) -> Result<Invitation> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/invitations", organization_id),
            invitation,
        )
    }

    /// Resend organization invitation
    pub fn resend_invitation(
        &self,
        organization_id: OrganizationId,
        invitation_id: u64,
    ) -> Result<()> {
        self.client.request_empty(
            Method::PUT,
            &format!(
                "organizations/{}/invitations/{}/resend",
                organization_id, invitation_id
            ),
        )
    }

    /// Accept organization invitation
    pub fn accept_invitation(&self, invitation_code: &str) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!("organizations/invitations/{}/accept", invitation_code),
        )
    }

    /// Reject organization invitation
    pub fn reject_invitation(&self, invitation_code: &str) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!("organizations/invitations/{}/reject", invitation_code),
        )
    }

    /// Leave organization
    pub fn leave(&self, organization_id: OrganizationId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("organizations/{}/users/leave", organization_id),
        )
    }

    /// Get detailed organization users
    pub fn get_users_detailed(
        &self,
        organization_id: OrganizationId,
        filter: Option<&str>,
        page: Option<u32>,
        per_page: Option<u32>,
        sort_field: Option<&str>,
        sort_order: Option<&str>,
    ) -> Result<Vec<OrganizationUserDetailed>> {
        let mut params = BTreeMap::new();
        if let Some(filter) = filter {
            params.insert("filter".to_string(), filter.to_string());
        }
        if let Some(page) = page {
            params.insert("page".to_string(), page.to_string());
        }
        if let Some(per_page) = per_page {
            params.insert("per_page".to_string(), per_page.to_string());
        }
        if let Some(sort_field) = sort_field {
            params.insert("sort_field".to_string(), sort_field.to_string());
        }
        if let Some(sort_order) = sort_order {
            params.insert("sort_order".to_string(), sort_order.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("organizations/{}/users/detailed", organization_id),
            &params,
        )
    }

    /// Update organization user
    pub fn update_user(
        &self,
        organization_id: OrganizationId,
        organization_user_id: UserId,
        update: &UpdateOrganizationUser,
    ) -> Result<OrganizationUser> {
        self.client.request_with_body(
            Method::PUT,
            &format!(
                "organizations/{}/users/{}",
                organization_id, organization_user_id
            ),
            update,
        )
    }

    /// Get organization roles
    pub fn get_roles(&self, organization_id: OrganizationId) -> Result<Vec<OrganizationRole>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/roles", organization_id),
        )
    }

    /// Get workspace statistics
    pub fn get_workspace_statistics(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<WorkspaceStatistics>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/workspaces/statistics", organization_id),
        )
    }

    /// Create workspace
    pub fn create_workspace(
        &self,
        organization_id: OrganizationId,
        workspace: &crate::models::api::workspace::CreateWorkspace,
    ) -> Result<crate::models::api::workspace::Workspace> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/workspaces", organization_id),
            workspace,
        )
    }

    /// Get ownership transfer requests
    pub fn get_ownership_transfers(
        &self,
        organization_id: OrganizationId,
    ) -> Result<Vec<OwnershipTransferRequest>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/owner/transfer", organization_id),
        )
    }

    /// Create ownership transfer request
    pub fn create_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer: &CreateOwnershipTransfer,
    ) -> Result<OwnershipTransferRequest> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/owner/transfer", organization_id),
            transfer,
        )
    }

    /// Get specific ownership transfer request
    pub fn get_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer_id: TransferId,
    ) -> Result<OwnershipTransferRequest> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/owner/transfer/{}",
                organization_id, transfer_id
            ),
        )
    }

    /// Accept or reject ownership transfer
    pub fn handle_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer_id: TransferId,
        action: &str,
    ) -> Result<()> {
        self.client.request_empty(
            Method::POST,
            &format!(
                "organizations/{}/owner/transfer/{}/{}",
                organization_id, transfer_id, action
            ),
        )
    }

    /// Accept ownership transfer
    pub fn accept_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer_id: TransferId,
    ) -> Result<()> {
        self.handle_ownership_transfer(organization_id, transfer_id, "accept")
    }

    /// Reject ownership transfer
    pub fn reject_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer_id: TransferId,
    ) -> Result<()> {
        self.handle_ownership_transfer(organization_id, transfer_id, "reject")
    }

    /// Cancel ownership transfer
    pub fn cancel_ownership_transfer(
        &self,
        organization_id: OrganizationId,
        transfer_id: TransferId,
    ) -> Result<()> {
        self.handle_ownership_transfer(organization_id, transfer_id, "cancel")
    }

    // Subscription Management

    /// Get organization subscription
    pub fn get_subscription(
        &self,
        organization_id: OrganizationId,
    ) -> Result<OrganizationSubscription> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/subscription", organization_id),
        )
    }

    /// Create organization subscription
    pub fn create_subscription(
        &self,
        organization_id: OrganizationId,
        subscription: &CreateOrganizationSubscription,
    ) -> Result<OrganizationSubscription> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/subscription", organization_id),
            subscription,
        )
    }

    /// Update organization subscription
    pub fn update_subscription(
        &self,
        organization_id: OrganizationId,
        subscription: &UpdateOrganizationSubscription,
    ) -> Result<OrganizationSubscription> {
        self.client.request_with_body(
            Method::PUT,
            &format!("organizations/{}/subscription", organization_id),
            subscription,
        )
    }

    /// Cancel organization subscription
    pub fn cancel_subscription(&self, organization_id: OrganizationId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("organizations/{}/subscription", organization_id),
        )
    }

    /// Submit cancellation feedback
    pub fn submit_cancellation_feedback(
        &self,
        organization_id: OrganizationId,
        feedback: &CancellationFeedback,
    ) -> Result<()> {
        self.client.request_with_body_empty(
            Method::POST,
            &format!(
                "organizations/{}/subscription/cancellation_feedback",
                organization_id
            ),
            feedback,
        )
    }

    /// Get subscription customer
    pub fn get_subscription_customer(&self, organization_id: OrganizationId) -> Result<Customer> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/subscription/customer", organization_id),
        )
    }

    /// Create subscription customer
    pub fn create_subscription_customer(
        &self,
        organization_id: OrganizationId,
        customer: &CreateCustomer,
    ) -> Result<Customer> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/subscription/customer", organization_id),
            customer,
        )
    }

    /// Update subscription customer
    pub fn update_subscription_customer(
        &self,
        organization_id: OrganizationId,
        customer: &UpdateCustomer,
    ) -> Result<Customer> {
        self.client.request_with_body(
            Method::PUT,
            &format!("organizations/{}/subscription/customer", organization_id),
            customer,
        )
    }

    /// Request discount
    pub fn request_discount(
        &self,
        organization_id: OrganizationId,
        request: &DiscountRequest,
    ) -> Result<SuccessResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/subscription/discount_request",
                organization_id
            ),
            request,
        )
    }

    /// Feature upsell multi
    pub fn feature_upsell_multi(
        &self,
        organization_id: OrganizationId,
        request: &FeatureUpsellRequest,
    ) -> Result<SuccessResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/subscription/feature_upsell_multi",
                organization_id
            ),
            request,
        )
    }

    /// Get invoice summary
    pub fn get_invoice_summary(&self, organization_id: OrganizationId) -> Result<InvoiceSummary> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/subscription/invoice_summary",
                organization_id
            ),
        )
    }

    /// Get payment failed status
    pub fn get_payment_failed(&self, organization_id: OrganizationId) -> Result<PaymentFailed> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/subscription/payment_failed",
                organization_id
            ),
        )
    }

    /// Apply promo code
    pub fn apply_promo_code(
        &self,
        organization_id: OrganizationId,
        promo_code: &OrganizationPromoCode,
    ) -> Result<SuccessResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/subscription/promocode", organization_id),
            promo_code,
        )
    }

    /// Remove promo code
    pub fn remove_promo_code(&self, organization_id: OrganizationId) -> Result<()> {
        self.client.request_empty(
            Method::DELETE,
            &format!("organizations/{}/subscription/promocode", organization_id),
        )
    }

    /// Get purchase order PDF
    pub fn get_purchase_order_pdf(
        &self,
        organization_id: OrganizationId,
        purchase_order_uid: &str,
    ) -> Result<Vec<u8>> {
        self.client.request_binary(
            Method::GET,
            &format!(
                "organizations/{}/subscription/purchase_orders/{}.pdf",
                organization_id, purchase_order_uid
            ),
        )
    }

    /// Apply referral bonus
    pub fn apply_referral_bonus(
        &self,
        organization_id: OrganizationId,
        referral: &ReferralBonus,
    ) -> Result<SuccessResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/subscription/referral_bonus",
                organization_id
            ),
            referral,
        )
    }

    /// Create setup intent
    pub fn create_setup_intent(&self, organization_id: OrganizationId) -> Result<SetupIntent> {
        self.client.request(
            Method::POST,
            &format!(
                "organizations/{}/subscription/setup_intent",
                organization_id
            ),
        )
    }

    /// Start trial
    pub fn start_trial(
        &self,
        organization_id: OrganizationId,
        trial: &StartTrial,
    ) -> Result<OrganizationSubscription> {
        self.client.request_with_body(
            Method::POST,
            &format!("organizations/{}/subscription/trial", organization_id),
            trial,
        )
    }

    /// Request upgrade
    pub fn request_upgrade(
        &self,
        organization_id: OrganizationId,
        feature_id: &str,
        request: &UpgradeRequest,
    ) -> Result<SuccessResponse> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/subscription/upgrade_request/{}",
                organization_id, feature_id
            ),
            request,
        )
    }

    /// Get payment records
    pub fn get_payment_records(
        &self,
        organization_id: OrganizationId,
        is_unified: Option<bool>,
    ) -> Result<Vec<PaymentRecord>> {
        let mut params = BTreeMap::new();
        if let Some(is_unified) = is_unified {
            params.insert("is_unified".to_string(), is_unified.to_string());
        }

        self.client.request_with_params(
            Method::GET,
            &format!("organizations/{}/payment_records", organization_id),
            &params,
        )
    }

    /// Get organization plans
    pub fn get_plans(&self, organization_id: OrganizationId) -> Result<Vec<OrganizationPlan>> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/plans", organization_id),
        )
    }

    /// Get specific organization plan
    pub fn get_plan(
        &self,
        organization_id: OrganizationId,
        plan_id: PricingPlanId,
    ) -> Result<OrganizationPlan> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/plans/{}", organization_id, plan_id),
        )
    }

    /// Get organization segmentation
    pub fn get_segmentation(
        &self,
        organization_id: OrganizationId,
    ) -> Result<OrganizationSegmentation> {
        self.client.request(
            Method::GET,
            &format!("organizations/{}/segmentation", organization_id),
        )
    }

    /// Update organization segmentation
    pub fn update_segmentation(
        &self,
        organization_id: OrganizationId,
        segmentation: &OrganizationSegmentation,
    ) -> Result<OrganizationSegmentation> {
        self.client.request_with_body(
            Method::PUT,
            &format!("organizations/{}/segmentation", organization_id),
            segmentation,
        )
    }

    /// Get workspace assignments
    pub fn get_workspace_assignments(
        &self,
        organization_id: OrganizationId,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<WorkspaceAssignment>> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/workspaces/{}/assignments",
                organization_id, workspace_id
            ),
        )
    }

    /// Create workspace assignment
    pub fn create_workspace_assignment(
        &self,
        organization_id: OrganizationId,
        workspace_id: WorkspaceId,
        assignment: &CreateWorkspaceAssignment,
    ) -> Result<WorkspaceAssignment> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/workspaces/{}/assignments",
                organization_id, workspace_id
            ),
            assignment,
        )
    }

    /// Get workspace groups
    pub fn get_workspace_groups(
        &self,
        organization_id: OrganizationId,
        workspace_id: WorkspaceId,
    ) -> Result<Vec<OrganizationGroup>> {
        self.client.request(
            Method::GET,
            &format!(
                "organizations/{}/workspaces/{}/groups",
                organization_id, workspace_id
            ),
        )
    }

    /// Create workspace group assignment
    pub fn create_workspace_group(
        &self,
        organization_id: OrganizationId,
        workspace_id: WorkspaceId,
        group_id: GroupId,
    ) -> Result<OrganizationGroup> {
        self.client.request_with_body(
            Method::POST,
            &format!(
                "organizations/{}/workspaces/{}/groups",
                organization_id, workspace_id
            ),
            &serde_json::json!({ "group_id": group_id }),
        )
    }
}
