use backlog_api_core::Result;
use backlog_core::ProjectIdOrKey;
use client::Client;

use crate::requests::{GetProjectListResponse, GetProjectParams, GetProjectResponse};

pub struct ProjectApi(Client);

impl ProjectApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    pub async fn get_project_list(
        &self,
        params: GetProjectParams,
    ) -> Result<GetProjectListResponse> {
        self.0.get_with_params("/api/v2/projects", &params).await
    }

    pub async fn get_project(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetProjectResponse> {
        self.0
            .get(&format!("/api/v2/projects/{}", project_id_or_key.into()))
            .await
    }
}
