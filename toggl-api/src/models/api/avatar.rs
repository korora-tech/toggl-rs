use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Avatar {
    pub url: String,
    pub url_large: String,
}
