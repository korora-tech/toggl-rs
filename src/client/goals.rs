use super::TogglClient;
use crate::{
    error::Result,
    model::api::{
        CreateGoalRequest, UpdateGoalRequest, WorkspaceGoal, WorkspaceGoalModel,
        WorkspaceGoalsQuery,
    },
};
use std::collections::BTreeMap;

pub struct GoalsClient {
    client: TogglClient,
}

impl GoalsClient {
    pub fn new(client: TogglClient) -> Self {
        Self { client }
    }
    /// Get workspace goals
    pub fn get_workspace_goals(
        &self,
        workspace_id: u64,
        query: Option<WorkspaceGoalsQuery>,
    ) -> Result<Vec<WorkspaceGoal>> {
        let url = format!("/workspaces/{}/goals", workspace_id);
        let mut params = BTreeMap::new();

        if let Some(q) = query {
            if let Some(team_goals) = q.team_goals {
                params.insert("team_goals".to_string(), team_goals.to_string());
            }
            if let Some(active) = q.active {
                params.insert("active".to_string(), active.to_string());
            }
            if let Some(page) = q.page {
                params.insert("page".to_string(), page.to_string());
            }
            if let Some(per_page) = q.per_page {
                params.insert("per_page".to_string(), per_page.to_string());
            }
        }

        self.client.get(&url, &params)
    }

    /// Create a workspace goal
    pub fn create_workspace_goal(
        &self,
        workspace_id: u64,
        goal: CreateGoalRequest,
    ) -> Result<WorkspaceGoalModel> {
        let url = format!("/workspaces/{}/goals", workspace_id);
        self.client.post(&url, &goal)
    }

    /// Get a specific workspace goal
    pub fn get_workspace_goal(&self, workspace_id: u64, goal_id: u64) -> Result<WorkspaceGoal> {
        let url = format!("/workspaces/{}/goals/{}", workspace_id, goal_id);
        self.client.get(&url, &BTreeMap::new())
    }

    /// Update a workspace goal
    pub fn update_workspace_goal(
        &self,
        workspace_id: u64,
        goal_id: u64,
        goal: UpdateGoalRequest,
    ) -> Result<WorkspaceGoalModel> {
        let url = format!("/workspaces/{}/goals/{}", workspace_id, goal_id);
        self.client.put(&url, &goal)
    }

    /// Delete a workspace goal
    pub fn delete_workspace_goal(&self, workspace_id: u64, goal_id: u64) -> Result<()> {
        let url = format!("/workspaces/{}/goals/{}", workspace_id, goal_id);
        self.client.delete_empty(&url)
    }
}
