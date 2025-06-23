use crate::models::Wiki;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetWikiListResponse = Vec<Wiki>;

#[derive(Debug, Clone)]
pub struct GetWikiListParams {
    pub project_id_or_key: Option<ProjectIdOrKey>,
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
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/wikis".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let mut query_params = Vec::new();
        if let Some(project_id_or_key) = &self.project_id_or_key {
            query_params.push(("projectIdOrKey".to_string(), project_id_or_key.to_string()));
        }
        if let Some(keyword) = &self.keyword {
            query_params.push(("keyword".to_string(), keyword.clone()));
        }
        query_params
    }
}
