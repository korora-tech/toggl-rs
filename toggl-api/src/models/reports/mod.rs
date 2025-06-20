pub mod comparative;
pub mod detailed;
pub mod export;
pub mod profitability;
pub mod summary;
pub mod trends;
pub mod weekly;

pub use comparative::*;
pub use detailed::*;
pub use export::*;
pub use profitability::*;
pub use summary::*;
pub use trends::*;
pub use weekly::*;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use toggl_core::{ClientId, ProjectId, TagId, TaskId, UserId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportTimeRange {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportFilters {
    pub client_ids: Option<Vec<ClientId>>,
    pub project_ids: Option<Vec<ProjectId>>,
    pub user_ids: Option<Vec<UserId>>,
    pub tag_ids: Option<Vec<TagId>>,
    pub task_ids: Option<Vec<TaskId>>,
    pub billable: Option<bool>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportGrouping {
    pub projects: Option<bool>,
    pub clients: Option<bool>,
    pub users: Option<bool>,
    pub tags: Option<bool>,
    pub tasks: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportOrdering {
    pub field: String,
    pub direction: OrderDirection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderDirection {
    Asc,
    Desc,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Pagination {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReportTotals {
    pub time: u64,
    pub billable_time: Option<u64>,
    pub amount: Option<f64>,
    pub billable_amount: Option<f64>,
    pub count: u32,
}
