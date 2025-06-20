#[cfg(test)]
mod tests {
    use super::super::*;
    use reqwest::Method;
    use serde_json::json;
    use std::collections::BTreeMap;
    use toggl_core::{Result, WorkspaceId};

    #[test]
    fn test_search_clients() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 1,
                    "name": "Client ABC",
                    "archived": false
                },
                {
                    "id": 2,
                    "name": "Client XYZ",
                    "archived": false
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("q", "Client");

        with_mockito_params(
            Method::GET,
            "/workspace/123/search/clients",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.search().clients(WorkspaceId(123), "Client")?;
                assert_eq!(result.data.len(), 2);
                assert_eq!(result.data[0].name, "Client ABC");
                Ok(())
            },
        )
    }

    #[test]
    fn test_search_projects() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 1,
                    "name": "Project Alpha",
                    "client_id": 123,
                    "active": true,
                    "billable": true,
                    "color": "#ff0000"
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("q", "Alpha");

        with_mockito_params(
            Method::GET,
            "/workspace/123/search/projects",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.search().projects(WorkspaceId(123), "Alpha")?;
                assert_eq!(result.data.len(), 1);
                assert_eq!(result.data[0].name, "Project Alpha");
                assert!(result.data[0].active);
                Ok(())
            },
        )
    }

    #[test]
    fn test_search_users() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "id": 1,
                    "name": "John Doe",
                    "email": "john@example.com",
                    "active": true
                },
                {
                    "id": 2,
                    "name": "Jane Smith",
                    "email": "jane@example.com",
                    "active": true
                }
            ]
        });

        let mut params = BTreeMap::new();
        params.insert("q", "John");

        with_mockito_params(
            Method::GET,
            "/workspace/123/search/users",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.search().users(WorkspaceId(123), "John")?;
                assert_eq!(result.data.len(), 2);
                assert_eq!(result.data[0].name, "John Doe");
                assert!(result.data[0].active);
                Ok(())
            },
        )
    }
}
