use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type UpdateCategoryResponse = backlog_domain_models::Category;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateCategoryParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub category_id: backlog_core::identifier::CategoryId,
    pub name: String,
}

#[cfg(feature = "writable")]
impl UpdateCategoryParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<backlog_core::identifier::CategoryId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            category_id: category_id.into(),
            name: name.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl From<&UpdateCategoryParams> for Vec<(String, String)> {
    fn from(params: &UpdateCategoryParams) -> Self {
        vec![("name".to_string(), params.name.clone())]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/categories/{}",
            self.project_id_or_key, self.category_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}