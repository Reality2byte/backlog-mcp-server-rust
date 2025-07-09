#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, User};
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Serialize)]
pub struct DeleteProjectUserParams {
    #[serde(skip)]
    pub project_id_or_key: ProjectIdOrKey,
    #[serde(rename = "userId")]
    pub user_id: u32,
}

#[cfg(feature = "writable")]
impl DeleteProjectUserParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, user_id: u32) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            user_id,
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteProjectUserParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/users", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        self
    }
}

#[cfg(feature = "writable")]
pub type DeleteProjectUserResponse = User;
