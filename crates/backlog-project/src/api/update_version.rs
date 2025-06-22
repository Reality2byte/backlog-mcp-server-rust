use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type UpdateVersionResponse = backlog_domain_models::Milestone;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateVersionParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub version_id: backlog_core::identifier::MilestoneId,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
    pub archived: Option<bool>,
}

#[cfg(feature = "writable")]
impl UpdateVersionParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        version_id: impl Into<backlog_core::identifier::MilestoneId>,
        name: impl Into<String>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            version_id: version_id.into(),
            name: name.into(),
            description: None,
            start_date: None,
            release_due_date: None,
            archived: None,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&UpdateVersionParams> for Vec<(String, String)> {
    fn from(params: &UpdateVersionParams) -> Self {
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

        if let Some(archived) = params.archived {
            seq.push(("archived".to_string(), archived.to_string()));
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateVersionParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/versions/{}",
            self.project_id_or_key, self.version_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}