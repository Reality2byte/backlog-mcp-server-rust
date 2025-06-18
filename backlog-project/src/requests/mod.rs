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
