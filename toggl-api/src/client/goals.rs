use super::TogglClient;
use crate::models::api::goals::{
    CreateGoalRequest, Goal, UpdateGoalRequest, WorkspaceGoal, WorkspaceGoalsQuery,
};
use std::collections::BTreeMap;
use toggl_core::Result;
use toggl_core::{GoalId, WorkspaceId};

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
        workspace_id: WorkspaceId,
        query: Option<WorkspaceGoalsQuery>,
    ) -> Result<Vec<WorkspaceGoal>> {
        let url = format!("/workspaces/{}/goals", workspace_id.value());
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
        workspace_id: WorkspaceId,
        goal: CreateGoalRequest,
    ) -> Result<Goal> {
        let url = format!("/workspaces/{}/goals", workspace_id.value());
        self.client.post(&url, &goal)
    }

    /// Get a specific workspace goal
    pub fn get_workspace_goal(
        &self,
        workspace_id: WorkspaceId,
        goal_id: GoalId,
    ) -> Result<WorkspaceGoal> {
        let url = format!(
            "/workspaces/{}/goals/{}",
            workspace_id.value(),
            goal_id.value()
        );
        self.client.get(&url, &BTreeMap::new())
    }

    /// Update a workspace goal
    pub fn update_workspace_goal(
        &self,
        workspace_id: WorkspaceId,
        goal_id: GoalId,
        goal: UpdateGoalRequest,
    ) -> Result<Goal> {
        let url = format!(
            "/workspaces/{}/goals/{}",
            workspace_id.value(),
            goal_id.value()
        );
        self.client.put(&url, &goal)
    }

    /// Delete a workspace goal
    pub fn delete_workspace_goal(&self, workspace_id: WorkspaceId, goal_id: GoalId) -> Result<()> {
        let url = format!(
            "/workspaces/{}/goals/{}",
            workspace_id.value(),
            goal_id.value()
        );
        self.client.delete_empty(&url)
    }
}
