use crate::client::Client;
use crate::error::Error;
use crate::responses::get_recent_updates_response::GetRecentUpdatesResponse;
use crate::responses::get_space_response::GetSpaceResponse;
use crate::types::{Project, ProjectIdOrKey, User};

pub async fn get_space(client: &Client) -> Result<GetSpaceResponse, Error> {
    client.get("/api/v2/space").await
}

pub async fn get_recent_updates(client: &Client) -> Result<GetRecentUpdatesResponse, Error> {
    client.get("/api/v2/space/activities").await
}

type GetProjectListResponse = Vec<Project>;
pub async fn get_project_list(
    client: &Client,
    params: GetProjectParams,
) -> Result<GetProjectListResponse, Error> {
    let params: Vec<(String, String)> = params.into();
    client.get_with_params("/api/v2/projects", &params).await
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Default)]
pub struct GetProjectParams {
    pub archived: Option<bool>,
    pub all: bool,
}

impl From<GetProjectParams> for Vec<(String, String)> {
    fn from(params: GetProjectParams) -> Self {
        let mut vec = Vec::new();
        if let Some(archived) = params.archived {
            vec.push(("archived".to_string(), archived.to_string()))
        }
        vec.push(("all".to_string(), params.all.to_string()));
        vec
    }
}

type GetProjectResponse = Project;
pub async fn get_project(
    client: &Client,
    project_id_or_key: ProjectIdOrKey,
) -> Result<GetProjectResponse, Error> {
    client
        .get(&format!("/api/v2/projects/{}", project_id_or_key))
        .await
}

type GetUserResponse = User;
pub async fn get_own_user(client: &Client) -> Result<GetUserResponse, Error> {
    client.get("/api/v2/users/myself").await
}
