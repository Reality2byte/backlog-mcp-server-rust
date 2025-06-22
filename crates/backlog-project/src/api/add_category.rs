use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type AddCategoryResponse = backlog_domain_models::Category;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddCategoryParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
}

#[cfg(feature = "writable")]
impl AddCategoryParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl From<&AddCategoryParams> for Vec<(String, String)> {
    fn from(params: &AddCategoryParams) -> Self {
        vec![("name".to_string(), params.name.clone())]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/categories", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
