use crate::models::Project;

pub type GetProjectListResponse = Vec<Project>;
pub type GetProjectResponse = Project;

#[derive(serde::Serialize, Debug, Default)]
pub struct GetProjectParams {
    pub archived: Option<bool>,
    pub all: bool,
}
