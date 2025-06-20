//! Event filter models

use serde::{Deserialize, Serialize};

/// Event filters output DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFiltersOutDto {
    /// Available event filter groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_filters: Option<Vec<EventFiltersGroupDto>>,
}

/// Event filter group DTO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventFiltersGroupDto {
    /// Available actions for this entity
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<String>>,

    /// Entity name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}
