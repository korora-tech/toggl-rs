//! API models and types
//!
//! This module organizes all API models into logical groups.
//! Instead of using wildcard re-exports, we use explicit exports
//! to avoid naming conflicts and make the API clearer.
//!
//! # Module Organization
//!
//! Models are organized by their API domain:
//! - `api::` - Core API request/response models
//! - `reports::` - Report-specific models
//!
//! # Common Types
//!
//! The most commonly used types are re-exported at the module root:
//! - IDs: `WorkspaceId`, `ProjectId`, `TimeEntryId`, etc.
//! - Core models: `User`, `Workspace`, `Project`, `TimeEntry`
//! - Create/Update types: `CreateProject`, `UpdateProject`, etc.
//!
//! # Handling Naming Conflicts
//!
//! Some types have naming conflicts between modules. These are organized
//! into sub-modules with clear namespacing:
//! - `organization::` - Organization-specific types
//! - `workspace_types::` - Workspace-specific types that conflict with organization
//! - `goals_types::` - Goal types from different modules
//! - `subscription_types::` - Subscription-related types

pub mod api {
    // Module declarations
    pub mod alert;
    pub mod audit;
    pub mod auth;
    pub mod avatar;
    pub mod billing;
    pub mod calendar;
    pub mod client;
    pub mod country;
    pub mod currency;
    pub mod dashboard;
    pub mod desktop_login;
    pub mod expense;
    pub mod exports;
    pub mod favorite;
    pub mod features;
    pub mod feedback;
    pub mod goals;
    pub mod group;
    pub mod invitation;
    pub mod invoices;
    pub mod keys;
    pub mod organization;
    pub mod organization_subscription;
    pub mod preferences;
    pub mod project;
    pub mod rates;
    pub mod saml;
    pub mod scheduled_reports;
    pub mod segmentation;
    pub mod shared_reports;
    pub mod smail;
    pub mod status;
    pub mod subscription;
    pub mod sync_server;
    pub mod tag;
    pub mod task;
    pub mod time_entry;
    pub mod time_entry_invitations;
    pub mod timeline;
    pub mod timesheets;
    pub mod timezones;
    pub mod user;
    pub mod workspace;
    pub mod workspace_subscription;
}

pub mod reports;

// Core re-exports - only the most commonly used types
pub use api::client::{Client, CreateClient, UpdateClient};
pub use api::project::{CreateProject, Project, UpdateProject};
pub use api::tag::{CreateTag, Tag as ApiTag, UpdateTag};
pub use api::task::{CreateTask, Task, UpdateTask};
pub use api::time_entry::{CreateTimeEntry, TimeEntry, UpdateTimeEntry};
pub use api::user::{Tag as UserTag, User};
pub use api::workspace::{CreateWorkspace, UpdateWorkspace, Workspace, WorkspaceUser};
pub use toggl_core::{
    AlertId, ApiKeyId, AssigneeId, AuditId, AvatarId, BookmarkId, CalendarId, CategoryId, ClientId,
    CompanyId, ContactDetailId, CountryId, CountrySubdivisionId, CreatorId, CurrencyCode,
    CurrencyId, CustomerId, DashboardId, ExpenseId, ExportId, FavoriteId, FeatureId, GoalId,
    GroupId, IntegrationId, InvitationId, InvitationItemId, InvoiceId, InvoiceInfoId,
    InvoiceItemId, InvoiceLineItemId, LevelId, OrganizationId, PaymentMethodId, PaymentRecordId,
    PricingPlanId, ProductId, ProjectGroupId, ProjectId, ProjectUserId, ProviderUserId, RateId,
    ReminderId, ReportId, RoleId, SamlConfigurationId, ScheduledReportId, SetupId, SmailId,
    SsoProfileId, SubscriptionId, SubscriptionPeriodId, TagId, TaskId, TimeEntryId,
    TimeEntryInvitationId, TimelineId, TransferId, UserId, WebhookId, WorkspaceId, WorkspaceUserId,
};

// Authentication and status
pub use api::auth::{SamlLoginRequest, SamlLoginResponse};
pub use api::status::ApiStatus;

// Organization types with namespace prefix to avoid conflicts
pub mod organization {
    pub use super::api::organization::*;
    pub use super::api::organization_subscription::{
        CardDetails as SubscriptionCardDetails, ContactDetail as SubscriptionContactDetail,
        InvoiceInfo as SubscriptionInvoiceInfo, OrganizationPlan, OrganizationSubscription,
        PaymentInfo as SubscriptionPaymentInfo,
    };
}

// Workspace types with clear namespacing
pub mod workspace_types {
    pub use super::api::workspace::{
        CardDetails, ContactDetail, PaymentDetails, WorkspaceStatistics, WorkspaceSubscription,
    };
    pub use super::api::workspace_subscription::{
        SubscriptionCardDetails as WorkspaceCardDetails,
        SubscriptionContactDetail as WorkspaceContactDetail, WorkspaceSubscriptionResponse,
    };
}

// Goals - handle naming conflict between goals and sync_server
pub mod goals_types {
    pub use super::api::goals::{
        CreateGoalRequest, Goal as WorkspaceGoal, UpdateGoalRequest, WorkspaceGoalsQuery,
    };
    pub use super::api::sync_server::Goal as SyncServerGoal;
}

// Subscription types with clear distinction
pub mod subscription_types {
    pub use super::api::subscription::{
        CreateSubscription, Invoice, PaymentDetails, PromoCode, Subscription, SubscriptionPlan,
        UpdateSubscription,
    };
}

// Export other modules directly without wildcards
pub use api::{
    alert, audit, avatar, billing, calendar, country, currency, dashboard, desktop_login, expense,
    exports, favorite, features, feedback, group, invitation, invoices, keys, preferences, rates,
    saml, scheduled_reports, segmentation, shared_reports, smail, time_entry_invitations, timeline,
    timesheets, timezones,
};
