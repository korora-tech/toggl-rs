use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timezone {
    pub name: String,
    pub offset: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimezoneOffset {
    pub timezone: String,
    pub offset: i32,
    pub offset_name: String,
}
