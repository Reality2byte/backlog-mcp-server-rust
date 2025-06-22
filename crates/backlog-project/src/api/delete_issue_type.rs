use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type DeleteIssueTypeResponse = backlog_domain_models::IssueType;

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteIssueTypeParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub issue_type_id: backlog_core::identifier::IssueTypeId,
    pub substitute_issue_type_id: backlog_core::identifier::IssueTypeId,
}

#[cfg(feature = "writable")]
impl DeleteIssueTypeParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
        substitute_issue_type_id: impl Into<backlog_core::identifier::IssueTypeId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            issue_type_id: issue_type_id.into(),
            substitute_issue_type_id: substitute_issue_type_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl From<&DeleteIssueTypeParams> for Vec<(String, String)> {
    fn from(params: &DeleteIssueTypeParams) -> Self {
        vec![(
            "substituteIssueTypeId".to_string(),
            params.substitute_issue_type_id.to_string(),
        )]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteIssueTypeParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
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