use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use toggl_core::{CategoryId, ExpenseId, ProjectId, TaskId, UserId, WorkspaceId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Expense {
    pub id: ExpenseId,
    pub workspace_id: WorkspaceId,
    pub user_id: UserId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub spent_at: NaiveDate,
    pub description: String,
    pub currency: String,
    pub amount: f64,
    pub category_id: CategoryId,
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
    pub workspace_id: WorkspaceId,
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub spent_at: NaiveDate,
    pub description: String,
    pub currency: String,
    pub amount: f64,
    pub category_id: CategoryId,
    pub billable: Option<bool>,
    pub payee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateExpense {
    pub project_id: Option<ProjectId>,
    pub task_id: Option<TaskId>,
    pub spent_at: Option<NaiveDate>,
    pub description: Option<String>,
    pub currency: Option<String>,
    pub amount: Option<f64>,
    pub category_id: Option<CategoryId>,
    pub billable: Option<bool>,
    pub payee: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ExpenseCategory {
    pub id: CategoryId,
    pub workspace_id: WorkspaceId,
    pub name: String,
    pub at: DateTime<Utc>,
}
