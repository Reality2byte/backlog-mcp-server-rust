use backlog_api_core::{HttpMethod, IntoDownloadRequest, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq)]
pub struct GetProjectIconParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectIconParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectIconParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/image", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

impl IntoDownloadRequest for GetProjectIconParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/image", self.project_id_or_key)
    }
}