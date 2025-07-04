use crate::models::TeamResponse;
use backlog_api_core::{HttpMethod, IntoRequest};
use serde::{Deserialize, Serialize};

/// Response type for listing teams.
pub type ListTeamsResponse = Vec<ListTeamResponse>;

/// Individual team in the list response, which includes displayOrder
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTeamResponse {
    #[serde(flatten)]
    pub team: TeamResponse,
    pub display_order: Option<i32>,
}

/// Parameters for listing teams.
///
/// # Required Permissions
/// - Administrator
/// - Project Administrator
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListTeamsParams {
    /// Sort order of the results.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<ListTeamsOrder>,
    /// Offset for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    /// Number of items to return (1-100, default: 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
}

/// Sort order for listing teams.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ListTeamsOrder {
    Asc,
    Desc,
}

impl IntoRequest for ListTeamsParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/teams".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
