use crate::models::WikiCount;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetWikiCountResponse = WikiCount;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWikiCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id_or_key: Option<ProjectIdOrKey>,
}

impl GetWikiCountParams {
    pub fn new() -> Self {
        Self {
            project_id_or_key: None,
        }
    }

    pub fn project_id_or_key(mut self, project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        self.project_id_or_key = Some(project_id_or_key.into());
        self
    }
}

impl Default for GetWikiCountParams {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoRequest for GetWikiCountParams {
    fn path(&self) -> String {
        "/api/v2/wikis/count".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
