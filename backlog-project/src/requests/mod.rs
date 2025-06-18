use crate::models::Project;

pub type GetProjectListResponse = Vec<Project>;
pub type GetProjectResponse = Project;

#[derive(serde::Serialize, Debug, Default)]
pub struct GetProjectParams {
    pub archived: Option<bool>,
    pub all: bool,
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddCategoryParams {
    pub name: String,
}

#[cfg(feature = "writable")]
impl From<&AddCategoryParams> for Vec<(String, String)> {
    fn from(params: &AddCategoryParams) -> Self {
        vec![("name".to_string(), params.name.clone())]
    }
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateCategoryParams {
    pub name: String,
}

#[cfg(feature = "writable")]
impl From<&UpdateCategoryParams> for Vec<(String, String)> {
    fn from(params: &UpdateCategoryParams) -> Self {
        vec![("name".to_string(), params.name.clone())]
    }
}
