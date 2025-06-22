use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type AddIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddIssueTypeParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub color: backlog_domain_models::IssueTypeColor,
    pub template_summary: Option<String>,
    pub template_description: Option<String>,
}

#[cfg(feature = "writable")]
impl AddIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        color: backlog_domain_models::IssueTypeColor,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            color,
            template_summary: None,
            template_description: None,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&AddIssueTypeParams> for Vec<(String, String)> {
    fn from(params: &AddIssueTypeParams) -> Self {
        let mut seq = vec![
            ("name".to_string(), params.name.clone()),
            ("color".to_string(), params.color.as_hex().to_string()),
        ];

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
impl IntoRequest for AddIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/issueTypes", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}