use backlog_api_core::IntoRequest;
use backlog_domain_models::Project;
use serde::Serialize;

pub type GetProjectListResponse = Vec<Project>;

// GET /api/v2/projects
#[derive(serde::Serialize, Debug, Default)]
pub struct GetProjectListParams {
    pub archived: Option<bool>,
    pub all: bool,
}

impl IntoRequest for GetProjectListParams {
    fn path(&self) -> String {
        "/api/v2/projects".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}
