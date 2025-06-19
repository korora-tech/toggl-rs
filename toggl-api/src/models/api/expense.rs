use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    pub id: u64,
    pub workspace_id: u64,
    pub user_id: u64,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub spent_at: NaiveDate,
    pub description: String,
    pub currency: String,
    pub amount: f64,
    pub category_id: u64,
    pub category_name: String,
    pub billable: bool,
    pub payee: Option<String>,
    pub receipt_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateExpense {
    pub workspace_id: u64,
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub spent_at: NaiveDate,
    pub description: String,
    pub currency: String,
    pub amount: f64,
    pub category_id: u64,
    pub billable: Option<bool>,
    pub payee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateExpense {
    pub project_id: Option<u64>,
    pub task_id: Option<u64>,
    pub spent_at: Option<NaiveDate>,
    pub description: Option<String>,
    pub currency: Option<String>,
    pub amount: Option<f64>,
    pub category_id: Option<u64>,
    pub billable: Option<bool>,
    pub payee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseCategory {
    pub id: u64,
    pub workspace_id: u64,
    pub name: String,
    pub at: DateTime<Utc>,
}
