use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetMilestoneListResponse = Vec<backlog_domain_models::Milestone>;

// GET /api/v2/projects/:projectIdOrKey/versions
#[derive(Debug, Clone, PartialEq)]
pub struct GetMilestoneListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetMilestoneListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetMilestoneListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/versions", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
