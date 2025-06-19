use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Country {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub vat_applicable: bool,
    pub vat_regex: Option<String>,
    pub vat_percentage: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Subdivision {
    pub id: u32,
    pub name: String,
    pub code: String,
    pub country_id: u32,
}
