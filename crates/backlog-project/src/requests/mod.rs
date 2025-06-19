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

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddIssueTypeParams {
    pub name: String,
    pub color: backlog_domain_models::IssueTypeColor,
    pub template_summary: Option<String>,
    pub template_description: Option<String>,
}

#[cfg(feature = "writable")]
impl From<&AddIssueTypeParams> for Vec<(String, String)> {
    fn from(params: &AddIssueTypeParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("name".to_string(), params.name.clone()));
        seq.push(("color".to_string(), params.color.as_hex().to_string()));

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
#[derive(Debug, Clone)]
pub struct DeleteIssueTypeParams {
    pub substitute_issue_type_id: backlog_core::identifier::IssueTypeId,
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
#[derive(Debug, Clone)]
pub struct UpdateIssueTypeParams {
    pub name: Option<String>,
    pub color: Option<backlog_domain_models::IssueTypeColor>,
    pub template_summary: Option<String>,
    pub template_description: Option<String>,
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
#[derive(Debug, Clone)]
pub struct AddVersionParams {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
}

#[cfg(feature = "writable")]
impl From<&AddVersionParams> for Vec<(String, String)> {
    fn from(params: &AddVersionParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("name".to_string(), params.name.clone()));

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
#[derive(Debug, Clone)]
pub struct UpdateVersionParams {
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
    pub archived: Option<bool>,
}

#[cfg(feature = "writable")]
impl From<&UpdateVersionParams> for Vec<(String, String)> {
    fn from(params: &UpdateVersionParams) -> Self {
        let mut seq = Vec::new();
        seq.push(("name".to_string(), params.name.clone()));

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
