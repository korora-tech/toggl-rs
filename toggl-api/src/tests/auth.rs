#[cfg(test)]
mod tests {
    use crate::models::api::ids::WorkspaceId;
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_saml2_login_without_workspace() -> Result<()> {
        let response = json!({
            "sso_url": "https://idp.example.com/saml/login?SAMLRequest=..."
        });

        with_mockito(
            Method::POST,
            "/auth/saml2/login",
            200,
            Some(response),
            |client| {
                let result = client.auth().saml2_login(None)?;

                assert_eq!(
                    result.sso_url,
                    "https://idp.example.com/saml/login?SAMLRequest=..."
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_saml2_login_with_workspace() -> Result<()> {
        let response = json!({
            "sso_url": "https://idp.example.com/saml/login?SAMLRequest=workspace123..."
        });

        with_mockito(
            Method::POST,
            "/auth/saml2/login/123456",
            200,
            Some(response),
            |client| {
                let result = client.auth().saml2_login(Some(WorkspaceId(123456)))?;

                assert_eq!(
                    result.sso_url,
                    "https://idp.example.com/saml/login?SAMLRequest=workspace123..."
                );
                Ok(())
            },
        )
    }
}
