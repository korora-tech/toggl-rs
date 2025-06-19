#[cfg(test)]
mod tests {
    use crate::{error::Result, model::api::project::ProjectGroupPayload, tests::with_mockito};
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_get_project_groups() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "group_id": 100,
                "project_id": 200,
                "workspace_id": 123
            },
            {
                "id": 2,
                "group_id": 101,
                "project_id": 201,
                "workspace_id": 123
            }
        ]);

        with_mockito(
            Method::GET,
            "/workspaces/123/project_groups",
            200,
            Some(response),
            |client| {
                let groups = client.workspaces().get_project_groups(123)?;
                assert_eq!(groups.len(), 2);
                assert_eq!(groups[0].id, 1);
                assert_eq!(groups[0].group_id, 100);
                assert_eq!(groups[0].project_id, 200);
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_project_group() -> Result<()> {
        let create_data = ProjectGroupPayload {
            project_id: 200,
            group_id: 100,
        };

        let response = json!({
            "id": 1,
            "group_id": 100,
            "project_id": 200,
            "workspace_id": 123
        });

        with_mockito(
            Method::POST,
            "/workspaces/123/project_groups",
            200,
            Some(response),
            |client| {
                let group = client
                    .workspaces()
                    .create_project_group(123, &create_data)?;
                assert_eq!(group.id, 1);
                assert_eq!(group.group_id, 100);
                assert_eq!(group.project_id, 200);
                assert_eq!(group.workspace_id, 123);
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_project_group() -> Result<()> {
        let update_data = ProjectGroupPayload {
            project_id: 202,
            group_id: 102,
        };

        let response = json!({
            "id": 1,
            "group_id": 102,
            "project_id": 202,
            "workspace_id": 123
        });

        with_mockito(
            Method::PUT,
            "/workspaces/123/project_groups/1",
            200,
            Some(response),
            |client| {
                let group = client
                    .workspaces()
                    .update_project_group(123, 1, &update_data)?;
                assert_eq!(group.id, 1);
                assert_eq!(group.group_id, 102);
                assert_eq!(group.project_id, 202);
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_project_group() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/workspaces/123/project_groups/1",
            200,
            None::<serde_json::Value>,
            |client| {
                client.workspaces().delete_project_group(123, 1)?;
                Ok(())
            },
        )
    }
}
