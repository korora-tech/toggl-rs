use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_list_currencies() -> Result<()> {
    let response = json!([
        {
            "code": "USD",
            "name": "United States Dollar",
            "symbol": "$"
        },
        {
            "code": "EUR",
            "name": "Euro",
            "symbol": "€"
        },
        {
            "code": "GBP",
            "name": "British Pound",
            "symbol": "£"
        }
    ]);

    with_mockito(Method::GET, "/currencies", 200, Some(response), |client| {
        let currencies = client.currencies().list()?;
        assert_eq!(3, currencies.len());
        assert_eq!("USD", currencies[0].code);
        assert_eq!("Euro", currencies[1].name);
        assert_eq!("£", currencies[2].symbol);
        Ok(())
    })
}
