use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Currency {
    pub code: String,
    pub name: String,
    pub symbol: String,
}
