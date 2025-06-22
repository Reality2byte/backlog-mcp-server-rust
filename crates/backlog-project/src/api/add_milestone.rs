use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type AddMilestoneResponse = backlog_domain_models::Milestone;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddMilestoneParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
}

#[cfg(feature = "writable")]
impl AddMilestoneParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>, name: impl Into<String>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            description: None,
            start_date: None,
            release_due_date: None,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&AddMilestoneParams> for Vec<(String, String)> {
    fn from(params: &AddMilestoneParams) -> Self {
        let mut seq = vec![("name".to_string(), params.name.clone())];

        if let Some(description) = &params.description {
            seq.push(("description".to_string(), description.clone()));
        }

        if let Some(start_date) = &params.start_date {
            seq.push(("startDate".to_string(), start_date.clone()));
        }

        if let Some(release_due_date) = &params.release_due_date {
            seq.push(("releaseDueDate".to_string(), release_due_date.clone()));
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddMilestoneParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/versions", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}
