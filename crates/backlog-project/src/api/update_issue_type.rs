use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type UpdateIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateIssueTypeParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub issue_type_id: backlog_core::identifier::IssueTypeId,
    pub name: Option<String>,
    pub color: Option<backlog_domain_models::IssueTypeColor>,
    pub template_summary: Option<String>,
    pub template_description: Option<String>,
}

#[cfg(feature = "writable")]
impl UpdateIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            issue_type_id: issue_type_id.into(),
            name: None,
            color: None,
            template_summary: None,
            template_description: None,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&UpdateIssueTypeParams> for Vec<(String, String)> {
    fn from(params: &UpdateIssueTypeParams) -> Self {
        let mut seq = Vec::new();

        if let Some(name) = &params.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(color) = &params.color {
            seq.push(("color".to_string(), color.as_hex().to_string()));
        }

        if let Some(template_summary) = &params.template_summary {
            seq.push(("templateSummary".to_string(), template_summary.clone()));
        }

        if let Some(template_description) = &params.template_description {
            seq.push((
                "templateDescription".to_string(),
                template_description.clone(),
            ));
        }

        seq
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/issueTypes/{}",
            self.project_id_or_key, self.issue_type_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}