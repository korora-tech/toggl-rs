//! Reports API models

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

// Re-export commonly used types
pub use base::*;
pub use comparative::*;
pub use data_trends::*;
pub use dictionary::*;
pub use filters::*;
pub use profitability::*;
pub use projects::*;
pub use search::*;
pub use shared::*;
pub use weekly::*;

// Re-export specific types to avoid conflicts
pub use detailed::{
    DetailedPost, DetailedReport, DetailedReportData, DetailedTotals,
    ExportPDFPost as DetailedExportPDFPost, GroupedTimeEntry, SearchExportPost, SingleTimeEntry,
};
pub use summary::{
    ExportPDFPost as SummaryExportPDFPost, ExportPost, Report, ReportData, ReportPost,
    ReportTotals, SubGroupData,
};
