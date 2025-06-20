use serde::{Deserialize, Serialize};

use super::ids::{CountryId, CountrySubdivisionId};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub id: CountryId,
    pub name: String,
    pub code: String,
    pub vat_applicable: bool,
    pub vat_regex: Option<String>,
    pub vat_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subdivision {
    pub id: CountrySubdivisionId,
    pub name: String,
    pub code: String,
    pub country_id: CountryId,
}
