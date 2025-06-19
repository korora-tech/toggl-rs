use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactRequest {
    pub name: String,
    pub email: String,
    pub company: Option<String>,
    pub company_size: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemoRequest {
    pub full_name: String,
    pub email: String,
    pub company_name: String,
    pub company_size: String,
    pub phone_number: Option<String>,
    pub country: String,
    pub additional_info: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetRequest {
    pub full_name: String,
    pub email: String,
    pub date: String,
    pub time: String,
    pub timezone: String,
    pub notes: Option<String>,
}
