#[cfg(test)]
mod tests {
    use crate::{
        error::Result,
        model::api::project::{CreateProjectUser, PatchOperation, UpdateProjectUser},
        tests::with_mockito,
    };
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_get_project_users() -> Result<()> {
        let response = json!([
            {
                "id": 1,
                "project_id": 200,
                "user_id": 300,
                "workspace_id": 123,
                "manager": true,
                "rate": 50.0,
                "rate_last_updated": "2023-01-01T10:00:00Z",
                "labour_cost": 45.0,
                "at": "2023-01-01T10:00:00Z",
                "group_ids": [1, 2]
            }
        ]);

        with_mockito(
            Method::GET,
            "/workspaces/123/project_users",
            200,
            Some(response),
            |client| {
                let users = client.workspaces().get_project_users(123)?;
                assert_eq!(users.len(), 1);
                assert_eq!(users[0].id, 1);
                assert_eq!(users[0].project_id, 200);
                assert_eq!(users[0].user_id, 300);
                assert!(users[0].manager);
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_project_user() -> Result<()> {
        let create_data = CreateProjectUser {
            project_id: 200,
            user_id: 300,
            manager: Some(true),
            rate: Some(50.0),
            labour_cost: Some(45.0),
        };

        let response = json!({
            "id": 1,
            "project_id": 200,
            "user_id": 300,
            "workspace_id": 123,
            "manager": true,
            "rate": 50.0,
            "rate_last_updated": "2023-01-01T10:00:00Z",
            "labour_cost": 45.0,
            "at": "2023-01-01T10:00:00Z",
            "group_ids": []
        });

        with_mockito(
            Method::POST,
            "/workspaces/123/project_users",
            200,
            Some(response),
            |client| {
                let user = client.workspaces().create_project_user(123, &create_data)?;
                assert_eq!(user.id, 1);
                assert_eq!(user.project_id, 200);
                assert_eq!(user.user_id, 300);
                assert!(user.manager);
                assert_eq!(user.rate, Some(50.0));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_project_user() -> Result<()> {
        let response = json!({
            "id": 1,
            "project_id": 200,
            "user_id": 300,
            "workspace_id": 123,
            "manager": false,
            "rate": 60.0,
            "rate_last_updated": "2023-01-01T10:00:00Z",
            "labour_cost": 55.0,
            "at": "2023-01-01T10:00:00Z",
            "group_ids": null
        });

        with_mockito(
            Method::GET,
            "/workspaces/123/project_users/1",
            200,
            Some(response),
            |client| {
                let user = client.workspaces().get_project_user(123, 1)?;
                assert_eq!(user.id, 1);
                assert!(!user.manager);
                assert_eq!(user.rate, Some(60.0));
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_project_user() -> Result<()> {
        let update_data = UpdateProjectUser {
            manager: Some(false),
            rate: Some(75.0),
            labour_cost: Some(70.0),
        };

        let response = json!({
            "id": 1,
            "project_id": 200,
            "user_id": 300,
            "workspace_id": 123,
            "manager": false,
            "rate": 75.0,
            "rate_last_updated": "2023-01-02T10:00:00Z",
            "labour_cost": 70.0,
            "at": "2023-01-01T10:00:00Z",
            "group_ids": null
        });

        with_mockito(
            Method::PUT,
            "/workspaces/123/project_users/1",
            200,
            Some(response),
            |client| {
                let user = client
                    .workspaces()
                    .update_project_user(123, 1, &update_data)?;
                assert_eq!(user.id, 1);
                assert!(!user.manager);
                assert_eq!(user.rate, Some(75.0));
                assert_eq!(user.labour_cost, Some(70.0));
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_project_user() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/workspaces/123/project_users/1",
            200,
            None::<serde_json::Value>,
            |client| {
                client.workspaces().delete_project_user(123, 1)?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_update_project_users_batch() -> Result<()> {
        let operations = vec![
            PatchOperation {
                op: "replace".to_string(),
                path: "/manager".to_string(),
                value: Some(json!(true)),
            },
            PatchOperation {
                op: "replace".to_string(),
                path: "/rate".to_string(),
                value: Some(json!(80.0)),
            },
        ];

        let response = json!([
            {
                "id": 1,
                "project_id": 200,
                "user_id": 300,
                "workspace_id": 123,
                "manager": true,
                "rate": 80.0,
                "rate_last_updated": "2023-01-03T10:00:00Z",
                "labour_cost": 45.0,
                "at": "2023-01-01T10:00:00Z",
                "group_ids": null
            },
            {
                "id": 2,
                "project_id": 200,
                "user_id": 301,
                "workspace_id": 123,
                "manager": true,
                "rate": 80.0,
                "rate_last_updated": "2023-01-03T10:00:00Z",
                "labour_cost": 45.0,
                "at": "2023-01-01T10:00:00Z",
                "group_ids": null
            }
        ]);

        with_mockito(
            Method::PATCH,
            "/workspaces/123/project_users/1,2",
            200,
            Some(response),
            |client| {
                let users =
                    client
                        .workspaces()
                        .update_project_users_batch(123, &[1, 2], &operations)?;
                assert_eq!(users.len(), 2);
                assert!(users[0].manager);
                assert_eq!(users[0].rate, Some(80.0));
                assert!(users[1].manager);
                assert_eq!(users[1].rate, Some(80.0));
                Ok(())
            },
        )
    }
}
