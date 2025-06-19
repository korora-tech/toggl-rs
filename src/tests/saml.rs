#[cfg(test)]
mod tests {
    use crate::model::api::saml::*;
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;
    use std::collections::BTreeMap;

    #[test]
    fn test_saml2_login() -> Result<()> {
        let response = json!({
            "sso_url": "https://idp.example.com/saml/login?SAMLRequest=..."
        });

        let mut params = BTreeMap::new();
        params.insert("email", "user%40example.com");

        with_mockito_params(
            Method::GET,
            "/auth/saml2/login",
            Some(params),
            200,
            Some(response),
            |client| {
                let result = client.saml().saml2_login("user@example.com")?;

                assert_eq!(
                    result.sso_url,
                    Some("https://idp.example.com/saml/login?SAMLRequest=...".to_string())
                );
                Ok(())
            },
        )
    }

    #[test]
    fn test_saml2_callback() -> Result<()> {
        with_mockito(Method::POST, "/auth/saml2/login/123", 200, None, |client| {
            client.saml().saml2_callback(
                123,
                "PHNhbWxwOlJlc3BvbnNlPi4uLjwvc2FtbHA6UmVzcG9uc2U+",
                Some("original-request-url"),
            )?;
            Ok(())
        })
    }

    #[test]
    fn test_enable_sso() -> Result<()> {
        let confirmation = SSOConfirmation {
            confirmation_code: "ABC123".to_string(),
        };

        with_mockito(Method::POST, "/me/enable_sso", 200, None, |client| {
            client.saml().enable_sso(&confirmation)?;
            Ok(())
        })
    }

    #[test]
    fn test_get_linked_sso_profiles() -> Result<()> {
        let response = json!([
            {
                "sso_profile_id": 1,
                "name": "Default SSO profile for Acme Corp",
                "domain": "acme.com"
            },
            {
                "sso_profile_id": 2,
                "name": "Secondary SSO profile",
                "domain": "acme.org"
            }
        ]);

        with_mockito(
            Method::GET,
            "/workspaces/123/linked_sso_profiles",
            200,
            Some(response),
            |client| {
                let profiles = client.saml().get_linked_sso_profiles(123)?;
                assert_eq!(profiles.len(), 2);
                assert_eq!(profiles[0].sso_profile_id, Some(1));
                assert_eq!(
                    profiles[0].name,
                    Some("Default SSO profile for Acme Corp".to_string())
                );
                assert_eq!(profiles[0].domain, Some("acme.com".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_link_sso_profile() -> Result<()> {
        let link_profile = LinkSsoProfile {
            domain: "acme.com".to_string(),
        };

        with_mockito(
            Method::PUT,
            "/workspaces/123/linked_sso_profiles/456",
            200,
            None,
            |client| {
                client.saml().link_sso_profile(123, 456, &link_profile)?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_unlink_sso_profile() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/workspaces/123/linked_sso_profiles/456",
            204,
            None,
            |client| {
                client.saml().unlink_sso_profile(123, 456)?;
                Ok(())
            },
        )
    }
}
