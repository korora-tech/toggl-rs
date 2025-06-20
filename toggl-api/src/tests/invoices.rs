#[cfg(test)]
mod tests {
    use crate::client::TogglClient;
    use crate::models::api::ids::{InvoiceId, OrganizationId, WorkspaceId};
    use crate::models::api::invoices::*;
    use crate::tests::*;
    use reqwest::Method;
    use serde_json::json;

    #[test]
    fn test_get_organization_invoice_pdf() -> Result<()> {
        use mockito::Server;

        let mut server = Server::new();
        let mock = server
            .mock("GET", "/organizations/123/invoices/INV-2024-001.pdf")
            .match_header("authorization", mockito::Matcher::Any)
            .with_status(200)
            .with_header("content-type", "application/pdf")
            .with_body(b"PDF_CONTENT_HERE")
            .create();

        let client = TogglClient::new_with_base_url("test_api_token".to_string(), &server.url())?;
        let pdf_bytes = client
            .invoices()
            .get_organization_invoice_pdf(OrganizationId(123), InvoiceId::from("INV-2024-001"))?;

        assert_eq!(pdf_bytes, b"PDF_CONTENT_HERE");
        mock.assert();

        Ok(())
    }

    #[test]
    fn test_get_organization_invoices() -> Result<()> {
        let response = json!({
            "items": [
                {
                    "id": "inv_123",
                    "invoice_number": "INV-2024-001",
                    "creation_date": "2024-01-15T10:00:00Z",
                    "currency": "USD",
                    "total_amount": 9900,
                    "status": "paid",
                    "description": "Monthly subscription",
                    "hosted_url": "https://example.com/invoice/inv_123",
                    "pdf_url": "https://example.com/invoice/inv_123.pdf"
                }
            ],
            "next": "cursor_123"
        });

        with_mockito(
            Method::GET,
            "/organizations/123/subscription/invoices",
            200,
            Some(response),
            |client| {
                let result = client
                    .invoices()
                    .get_organization_invoices(OrganizationId(123), None)?;
                assert_eq!(result.items.as_ref().unwrap().len(), 1);
                let invoice = &result.items.as_ref().unwrap()[0];
                assert_eq!(invoice.id, Some("inv_123".to_string()));
                assert_eq!(invoice.total_amount, Some(9900));
                assert_eq!(result.next, Some("cursor_123".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_organization_invoice_summary() -> Result<()> {
        let response = json!("Total invoices: 5, Total amount: $495.00");

        with_mockito(
            Method::GET,
            "/organizations/123/subscription/invoice_summary",
            200,
            Some(response),
            |client| {
                let summary = client
                    .invoices()
                    .get_organization_invoice_summary(OrganizationId(123))?;
                assert_eq!(summary, "Total invoices: 5, Total amount: $495.00");
                Ok(())
            },
        )
    }

    #[test]
    fn test_get_workspace_invoices() -> Result<()> {
        let response = json!({
            "data": [
                {
                    "user_invoice_id": 1,
                    "workspace_id": 456,
                    "document_id": "DOC-001",
                    "date": "2024-01-15",
                    "due_date": "2024-02-15",
                    "currency": "USD",
                    "billing_address": "123 Main St",
                    "workspace_address": "456 Work Ave",
                    "items": [
                        {
                            "item_id": 1,
                            "description": "Consulting services",
                            "quantity": 10.0,
                            "amount": 1000.0
                        }
                    ],
                    "taxes": [
                        {
                            "tax_id": 1,
                            "name": "VAT",
                            "amount": 100.0
                        }
                    ]
                }
            ],
            "page": 1,
            "per_page": 50,
            "total_count": 1
        });

        with_mockito(
            Method::GET,
            "/workspaces/456/invoices",
            200,
            Some(response),
            |client| {
                let result = client.invoices().get_workspace_invoices(
                    WorkspaceId(456),
                    None,
                    None,
                    None,
                    None,
                )?;
                assert_eq!(result.page, Some(1));
                assert_eq!(result.total_count, Some(1));
                assert_eq!(result.data.as_ref().unwrap().len(), 1);

                let invoice = &result.data.as_ref().unwrap()[0];
                assert_eq!(invoice.user_invoice_id, Some(1));
                assert_eq!(invoice.document_id, Some("DOC-001".to_string()));

                let items = invoice.items.as_ref().unwrap();
                assert_eq!(items.len(), 1);
                assert_eq!(items[0].amount, Some(1000.0));

                let taxes = invoice.taxes.as_ref().unwrap();
                assert_eq!(taxes.len(), 1);
                assert_eq!(taxes[0].name, Some("VAT".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_create_workspace_invoice() -> Result<()> {
        let new_invoice = CreateUserInvoice {
            billing_address: Some("123 Main St".to_string()),
            currency: Some("USD".to_string()),
            date: Some("2024-01-15".to_string()),
            due_date: Some("2024-02-15".to_string()),
            integration_ext_id: None,
            integration_ext_type: None,
            integration_provider: None,
            items: Some(vec![UserInvoiceItem {
                item_id: None,
                description: Some("Consulting services".to_string()),
                quantity: Some(10.0),
                amount: Some(1000.0),
            }]),
            message: Some("Thank you for your business".to_string()),
            payment_terms: Some("Net 30".to_string()),
            purchase_number: Some("PO-123".to_string()),
            taxes: Some(vec![UserInvoiceTax {
                tax_id: None,
                name: Some("VAT".to_string()),
                amount: Some(100.0),
            }]),
            workspace_address: Some("456 Work Ave".to_string()),
            workspace_logo: None,
        };

        let response = json!({
            "user_invoice_id": 1,
            "workspace_id": 456,
            "document_id": "DOC-001",
            "date": "2024-01-15",
            "due_date": "2024-02-15",
            "currency": "USD",
            "billing_address": "123 Main St",
            "created_at": "2024-01-15T10:00:00Z"
        });

        with_mockito(
            Method::POST,
            "/workspaces/456/invoices",
            201,
            Some(response),
            |client| {
                let invoice = client
                    .invoices()
                    .create_workspace_invoice(WorkspaceId(456), &new_invoice)?;
                assert_eq!(invoice.user_invoice_id, Some(1));
                assert_eq!(invoice.document_id, Some("DOC-001".to_string()));
                assert_eq!(invoice.billing_address, Some("123 Main St".to_string()));
                Ok(())
            },
        )
    }

    #[test]
    fn test_delete_workspace_invoice() -> Result<()> {
        with_mockito(
            Method::DELETE,
            "/workspaces/456/invoices/1",
            204,
            None,
            |client| {
                client
                    .invoices()
                    .delete_workspace_invoice(WorkspaceId(456), 1)?;
                Ok(())
            },
        )
    }

    #[test]
    fn test_invoice_with_integration() -> Result<()> {
        let invoice_with_integration = CreateUserInvoice {
            billing_address: Some("123 Main St".to_string()),
            currency: Some("USD".to_string()),
            date: Some("2024-01-15".to_string()),
            due_date: Some("2024-02-15".to_string()),
            integration_ext_id: Some("JIRA-123".to_string()),
            integration_ext_type: Some("issue".to_string()),
            integration_provider: Some(IntegrationProvider::Jira),
            items: Some(vec![]),
            message: None,
            payment_terms: None,
            purchase_number: None,
            taxes: None,
            workspace_address: None,
            workspace_logo: None,
        };

        let response = json!({
            "user_invoice_id": 2,
            "integration_ext_id": "JIRA-123",
            "integration_ext_type": "issue",
            "integration_provider": "jira"
        });

        with_mockito(
            Method::POST,
            "/workspaces/456/invoices",
            201,
            Some(response),
            |client| {
                let invoice = client
                    .invoices()
                    .create_workspace_invoice(WorkspaceId(456), &invoice_with_integration)?;
                assert_eq!(invoice.integration_ext_id, Some("JIRA-123".to_string()));
                assert_eq!(
                    invoice.integration_provider,
                    Some(IntegrationProvider::Jira)
                );
                Ok(())
            },
        )
    }
}
