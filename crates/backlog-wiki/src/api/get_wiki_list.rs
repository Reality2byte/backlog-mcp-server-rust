use crate::models::Wiki;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetWikiListResponse = Vec<Wiki>;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWikiListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id_or_key: Option<ProjectIdOrKey>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<String>,
}

impl GetWikiListParams {
    pub fn new() -> Self {
        Self {
            project_id_or_key: None,
            keyword: None,
        }
    }

    pub fn project_id_or_key(mut self, project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        self.project_id_or_key = Some(project_id_or_key.into());
        self
    }

    pub fn keyword(mut self, keyword: impl Into<String>) -> Self {
        self.keyword = Some(keyword.into());
        self
    }
}

impl Default for GetWikiListParams {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoRequest for GetWikiListParams {
    fn path(&self) -> String {
        "/api/v2/wikis".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
