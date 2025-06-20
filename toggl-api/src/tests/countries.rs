use crate::models::api::ids::CountryId;
use pretty_assertions::assert_eq;
use reqwest::Method;
use serde_json::json;
use toggl_core::Result;

use super::with_mockito;

#[test]
fn test_list_countries() -> Result<()> {
    let response = json!([
        {
            "id": 1,
            "name": "United States",
            "code": "US",
            "vat_applicable": false,
            "vat_regex": null,
            "vat_percentage": null
        },
        {
            "id": 2,
            "name": "Canada",
            "code": "CA",
            "vat_applicable": true,
            "vat_regex": null,
            "vat_percentage": 5.0
        }
    ]);

    with_mockito(Method::GET, "/countries", 200, Some(response), |client| {
        let countries = client.countries().list()?;
        assert_eq!(2, countries.len());
        assert_eq!("United States", countries[0].name);
        assert_eq!("CA", countries[1].code);
        Ok(())
    })
}

#[test]
fn test_get_subdivisions() -> Result<()> {
    let response = json!([
        {
            "id": 101,
            "name": "California",
            "code": "CA",
            "country_id": 1
        },
        {
            "id": 102,
            "name": "New York",
            "code": "NY",
            "country_id": 1
        }
    ]);

    with_mockito(
        Method::GET,
        "/countries/1/subdivisions",
        200,
        Some(response),
        |client| {
            let subdivisions = client.countries().get_subdivisions(CountryId(1))?;
            assert_eq!(2, subdivisions.len());
            assert_eq!("California", subdivisions[0].name);
            assert_eq!("NY", subdivisions[1].code);
            Ok(())
        },
    )
}
