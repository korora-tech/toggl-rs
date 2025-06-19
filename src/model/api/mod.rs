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
pub mod expense;
pub mod exports;
pub mod favorite;
pub mod features;
pub mod feedback;
pub mod goals;
pub mod group;
pub mod ids;
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

// Re-export ID types
pub use ids::{
    AlertId, ApiKeyId, AssigneeId, AvatarId, CalendarId, ClientId, CountryId, CreatorId,
    CurrencyId, DashboardId, ExportId, GoalId, GroupId, InvitationId, InvoiceId, OrganizationId,
    PricingPlanId, ProjectId, ProviderUserId, SamlConfigurationId, SmailId, SubscriptionId, TagId,
    TaskId, TimeEntryId, TimelineId, UserId, WorkspaceId,
};

// Re-export specific types to avoid conflicts
pub use alert::{Alert, AlertMeta, AlertWithMeta, CreateAlert, UpdateAlert};
pub use audit::AuditLog;
pub use auth::{SamlLoginRequest, SamlLoginResponse};
pub use avatar::Avatar;
pub use billing::{
    FancyPlan, FancyPlanFeature, PaymentCard, PaymentMethod, Plan, PlanFeature, PricingPlans,
    SEPADebit, USBankAccount,
};
pub use calendar::{
    Calendar, CalendarEvent, CalendarUpdate, CalendarUpdateRequest, CalendarsResponse,
    EventDetailsSuggestion, EventsResponse, Integration, UpdateIntegration,
};
pub use client::{ArchiveClientsResponse, BulkArchiveClients, Client, CreateClient, UpdateClient};
pub use country::{Country, Subdivision};
pub use currency::Currency;
pub use dashboard::{
    Activity, AllActivity, MostActive, MostActiveUser, TopActivities, TopActivity,
};
pub use expense::{CreateExpense, Expense, ExpenseCategory, UpdateExpense};
pub use exports::{ExportRequest, ExportStatus};
pub use favorite::{CreateFavorite, Favorite, UpdateFavorite};
pub use features::{Feature, Features, Location, TrackReminder};
pub use feedback::{FeedbackRequest, WebFeedbackRequest};
pub use goals::{
    CreateGoalRequest, Goal as WorkspaceGoalModel, UpdateGoalRequest, WorkspaceGoal,
    WorkspaceGoalsQuery,
};
pub use group::{
    CreateGroup, Group, GroupUser, PatchFailure, PatchGroupUsersInput, PatchGroupUsersOutput,
    UpdateGroup,
};
pub use invitation::{
    AcceptInvitation, CreateInvitation, Invitation, ResendInvitation, SSOInvitation,
};
pub use invoices::{
    CreateUserInvoice, IntegrationProvider, UnifiedSubscriptionInvoice,
    UnifiedSubscriptionInvoiceList, UserInvoice, UserInvoiceItem, UserInvoiceTax,
    UserInvoicesResponse,
};
pub use keys::ApiKey;
pub use organization::{
    CreateOrganization, CreateOwnershipTransfer, Organization, OrganizationGroup,
    OrganizationOwner, OrganizationRole, OrganizationUser, OrganizationUserDetailed,
    OrganizationWorkspace, OwnershipTransferRequest, TrialInfo, UpdateOrganization,
    UpdateOrganizationUser, WorkspaceStatistics,
};
pub use organization_subscription::{
    CancellationFeedback, CreateCustomer, CreateOrganizationSubscription, Customer,
    DiscountRequest, FeatureUpsellRequest, InvoiceSummary, OrganizationPlan, OrganizationPromoCode,
    OrganizationSubscription, PaymentFailed, PaymentRecord, ReferralBonus, SetupIntent, StartTrial,
    SuccessResponse, UpdateCustomer, UpdateOrganizationSubscription, UpgradeRequest,
};
pub use preferences::{
    AllPreferences, AlphaFeature, ClientPreferences, Flags, Logo, PushService, Quota,
    SharedTimeEntry, TimeEntryConstraints, WorkspacePreferences,
};
pub use project::{
    CreateProject, CreateProjectUser, CurrentPeriod, PatchOperation, Project, ProjectGroup,
    ProjectGroupPayload, ProjectIds, ProjectPeriod, ProjectStatistics, ProjectTemplate,
    ProjectUser, RecurringParameter, UpdateProject, UpdateProjectUser,
};
pub use rates::{CreateRate, Rate, RateLevel, RateMode};
pub use saml::{LinkSsoProfile, LinkedSsoProfile, LoginResponse, SSOConfirmation};
pub use scheduled_reports::{CreateScheduledReportPayload, ScheduledReport};
pub use segmentation::{CreateWorkspaceAssignment, OrganizationSegmentation, WorkspaceAssignment};
pub use shared_reports::{
    BulkDeleteRequest, CreateSavedReportPayload, SavedReport, SharedReportsQuery,
    UpdateSavedReportPayload,
};
pub use smail::{ContactRequest, DemoRequest, MeetRequest};
pub use status::ApiStatus;
pub use subscription::{
    ApplyCoupon,
    // Note: SubscriptionPeriod and PaymentDetails are kept in subscription module
    // to avoid conflicts with workspace module
    CreateSubscription,
    Invoice,
    PromoCode,
    Subscription,
    SubscriptionPlan,
    UpdateSubscription,
};
pub use sync_server::Goal;
pub use tag::{CreateTag, Tag as ApiTag, UpdateTag}; // Renamed to avoid conflict with user::Tag
pub use task::{BulkDeleteTasks, CreateTask, Task, UpdateTask};
pub use time_entry::{
    BulkEditOperation, BulkEditTimeEntries, CreateTimeEntry, TimeEntry, TimeEntrySharedWith,
    UpdateTimeEntry,
};
pub use time_entry_invitations::{InvitationAction, TimeEntryInvitation};
pub use timeline::{CreateTimelineEvent, TimelineEvent};
pub use timesheets::{
    APITimesheet, APITimesheetSetup, CreateTimesheetSetupPayload, PostTimesheetHoursPayload,
    PutBatchTimesheetPayload, PutTimesheetPayload, Review, Timesheet, TimesheetApprover,
    TimesheetError, TimesheetFilter, TimesheetHoursResponse, TimesheetSetupApprovers,
    TimesheetSetupError, TimesheetSetupsGetPaginatedResponse, TimesheetsGetPaginatedResponse,
    UpdateTimesheetSetupPayload, Weekday,
};
pub use timezones::{Timezone, TimezoneOffset};
pub use user::{
    LostPassword,
    ResetToken,
    SignupData,
    // Note: user::Tag is not re-exported to avoid conflict
    UpdateUser,
    User,
    UserWebTimer,
};
pub use workspace::{
    CSVUpload,
    CardDetails,
    ContactDetail,
    CreateWorkspace,
    UpdateWorkspace,
    Workspace,
    WorkspaceStatistics as WorkspaceStats,
    // Note: PaymentDetails and SubscriptionPeriod from workspace are not re-exported
    // to avoid conflicts with subscription module
    WorkspaceSubscription,
    WorkspaceUser,
};
pub use workspace_subscription::WorkspaceSubscriptionResponse;
