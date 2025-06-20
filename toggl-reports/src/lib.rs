//! # Toggl Reports API Client
//!
//! Client library for the Toggl Track Reports API.
//!
//! This crate provides access to Toggl's reporting functionality including:
//! - Summary reports
//! - Detailed reports
//! - Weekly reports
//! - Time entry reports
//! - Project and task analytics
//! - Comparative reports
//! - Data trends
//! - Profitability analysis
//! - Insights API

pub mod client;
pub mod models;

#[cfg(test)]
mod tests;

pub use client::ReportsClient;
pub use toggl_core::{Error, Result};

// Re-export model modules
pub mod prelude {
    pub use crate::models::base::*;
    pub use crate::models::comparative::*;
    pub use crate::models::data_trends::*;
    pub use crate::models::dictionary::*;
    pub use crate::models::filters::*;
    pub use crate::models::profitability::*;
    pub use crate::models::projects::*;
    pub use crate::models::search::*;
    pub use crate::models::shared::*;
    pub use crate::models::weekly::*;

    // Export specific types to avoid conflicts
    pub use crate::models::detailed::{
        DetailedPost, DetailedReport, ExportPDFPost as DetailedExportPDFPost, SearchExportPost,
    };
    pub use crate::models::summary::{
        ExportPDFPost as SummaryExportPDFPost, ExportPost as SummaryExportPost,
        Report as SummaryReport, ReportPost as SummaryReportPost,
    };
}
