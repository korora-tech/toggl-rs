//! Reports API models
//!
//! This module organizes all Reports API models into logical groups.
//! Instead of using wildcard re-exports, we use explicit exports
//! to avoid naming conflicts and make the API clearer.
//!
//! # Module Organization
//!
//! Models are organized by report type:
//! - `base` - Common base types used across reports
//! - `comparative` - Comparative report models
//! - `data_trends` - Data trends report models
//! - `detailed` - Detailed time entries report models
//! - `dictionary` - Dictionary/search models
//! - `filters` - Report filter models
//! - `profitability` - Profitability report models
//! - `projects` - Project report models
//! - `search` - Search-related models
//! - `shared` - Shared report models
//! - `summary` - Summary report models
//! - `weekly` - Weekly report models
//!
//! # Common Types
//!
//! The most commonly used types are re-exported at the module root:
//! - Base types: `Post`, `GroupingOption`, `OrderDirection`, etc.
//! - Report types: `DetailedReport`, `Report` (Summary), `WeeklyReport`, etc.
//!
//! # Handling Naming Conflicts
//!
//! Some types have naming conflicts between modules. These are handled by:
//! - Using explicit re-exports with renamed types (e.g., `DetailedExportPDFPost`, `SummaryExportPDFPost`)
//! - Grouping related types into sub-modules with clear namespacing

pub mod base;
pub mod comparative;
pub mod data_trends;
pub mod detailed;
pub mod dictionary;
pub mod filters;
pub mod profitability;
pub mod projects;
pub mod search;
pub mod shared;
pub mod summary;
pub mod weekly;

// Core re-exports - only the most commonly used types

// Base types
pub use base::{
    BillableHourlyRate, DataTrendsPost, ErrorResponse, GroupingOption, OrderDirection, Post,
    RangePost, SubGroupingOption,
};

// Comparative report
pub use comparative::{
    ComparativeData, ComparativePost, ComparativeReport, GraphData, ReportGraph,
};

// Data trends
pub use data_trends::{
    ClientDataTrend, ClientDataTrendsReport, DataTrendsGraph, DataTrendsGraphData,
    ProjectDataTrend, ProjectDataTrendsReport, UserDataTrend, UserDataTrendsReport,
};

// Dictionary types
pub use dictionary::{
    ClientDict, ClientDictionary, GeneralDictionary, GroupDict, ProjectDict, ProjectDictionary,
    ProjectUserDict, ReportDictionaries, ReportDictionariesData, TagDict, TaskDict, TaskDictionary,
    UserDict, UserDictionary,
};

// Filter types
pub use filters::{
    ClientFilterParamsRequest, ClientFilterResponse, ProjectFilterParamRequest,
    ProjectFilterResponse, ProjectGroupParamsRequest, ProjectGroupResponse,
    ProjectStatusParamsRequest, ProjectStatusResponse, ProjectUserParamsRequest,
    ProjectUserResponse, ProjectUsersRequest, Task, TaskStatusParamsRequest, TaskStatusResponse,
    TasksRequest, UserFilterParamsRequest, UserFilterResponse,
};

// Profitability
pub use profitability::{
    EmployeeProfitability, EmployeeProfitabilityRequest, ProfitabilityFilter, ProjectProfitability,
    ProjectProfitabilityReport, ProjectProfitabilityRequest, ProjectTrends, ProjectTrendsRequest,
    TrendDataPoint,
};

// Project reports
pub use projects::{
    ProjectSummaryData, ProjectSummaryRequest, ProjectSummaryResponse, ProjectsSummaryResponse,
};

// Search types
pub use search::{
    ClientSearchResult, ProjectSearchResult, SearchClientsResponse, SearchParams,
    SearchProjectsResponse, SearchUsersResponse, UserSearchResult,
};

// Shared reports
pub use shared::{SharedReportExport, SharedReportResponse};

// Weekly report
pub use weekly::{
    CurrencyAmount, UserDetails, WeekEntry, WeekTotal, WeeklyData, WeeklyDetail,
    WeeklyExportPDFPost, WeeklyExportPost, WeeklyPost, WeeklyReport,
};

// Detailed report types with conflict resolution
pub use detailed::{
    DetailedPost, DetailedReport, DetailedReportData, DetailedTotals,
    ExportPDFPost as DetailedExportPDFPost, GroupedTimeEntry, SearchExportPost, SingleTimeEntry,
};

// Summary report types with conflict resolution
pub use summary::{
    ExportPDFPost as SummaryExportPDFPost, ExportPost as SummaryExportPost,
    Report as SummaryReport, ReportData as SummaryReportData, ReportPost as SummaryReportPost,
    ReportTotals as SummaryReportTotals, SubGroupData as SummarySubGroupData,
};

// Export modules for advanced usage
pub use {
    base as base_types, comparative as comparative_types, data_trends as data_trends_types,
    detailed as detailed_types, dictionary as dictionary_types, filters as filters_types,
    profitability as profitability_types, projects as projects_types, search as search_types,
    shared as shared_types, summary as summary_types, weekly as weekly_types,
};
