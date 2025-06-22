use crate::models::Project;
use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetProjectListResponse = Vec<Project>;
pub type GetProjectResponse = Project;
pub type GetVersionMilestoneListResponse = Vec<backlog_domain_models::Milestone>;

// GET /api/v2/projects/:projectIdOrKey
#[derive(Debug, Clone, PartialEq)]
pub struct GetProjectDetailParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetProjectDetailParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetProjectDetailParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/projects
#[derive(serde::Serialize, Debug, Default)]
pub struct GetProjectListParams {
    pub archived: Option<bool>,
    pub all: bool,
}

impl IntoRequest for GetProjectListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/projects".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

// GET /api/v2/projects/:projectIdOrKey/statuses
#[derive(Debug, Clone, PartialEq)]
pub struct GetStatusListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetStatusListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetStatusListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/statuses", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/projects/:projectIdOrKey/issueTypes
#[derive(Debug, Clone, PartialEq)]
pub struct GetIssueTypeListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetIssueTypeListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetIssueTypeListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/issueTypes", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/projects/:projectIdOrKey/versions
#[derive(Debug, Clone, PartialEq)]
pub struct GetVersionMilestoneListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetVersionMilestoneListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetVersionMilestoneListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/versions", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/projects/:projectIdOrKey/categories
#[derive(Debug, Clone, PartialEq)]
pub struct GetCategoryListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl GetCategoryListParams {
    pub fn new(project_id_or_key: impl Into<ProjectIdOrKey>) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetCategoryListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/categories", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/priorities
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetPriorityListParams;

impl IntoRequest for GetPriorityListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/priorities".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

// GET /api/v2/resolutions
#[derive(Debug, Clone, Default, PartialEq)]
pub struct GetResolutionListParams;

impl IntoRequest for GetResolutionListParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        "/api/v2/resolutions".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

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

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteCategoryParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub category_id: backlog_core::identifier::CategoryId,
}

#[cfg(feature = "writable")]
impl DeleteCategoryParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        category_id: impl Into<backlog_core::identifier::CategoryId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            category_id: category_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteCategoryParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/categories/{}",
            self.project_id_or_key, self.category_id
        )
    }
}

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

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddVersionParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub description: Option<String>,
    pub start_date: Option<String>,
    pub release_due_date: Option<String>,
}

#[cfg(feature = "writable")]
impl AddVersionParams {
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
impl From<&AddVersionParams> for Vec<(String, String)> {
    fn from(params: &AddVersionParams) -> Self {
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
impl IntoRequest for AddVersionParams {
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

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteVersionParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub version_id: backlog_core::identifier::MilestoneId,
}

#[cfg(feature = "writable")]
impl DeleteVersionParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        version_id: impl Into<backlog_core::identifier::MilestoneId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            version_id: version_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteVersionParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/versions/{}",
            self.project_id_or_key, self.version_id
        )
    }
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct AddStatusParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub name: String,
    pub color: backlog_domain_models::StatusColor,
}

#[cfg(feature = "writable")]
impl AddStatusParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        name: impl Into<String>,
        color: backlog_domain_models::StatusColor,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            name: name.into(),
            color,
        }
    }
}

#[cfg(feature = "writable")]
impl From<&AddStatusParams> for Vec<(String, String)> {
    fn from(params: &AddStatusParams) -> Self {
        vec![
            ("name".to_string(), params.name.clone()),
            ("color".to_string(), params.color.as_hex().to_string()),
        ]
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for AddStatusParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/statuses", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        Vec::<(String, String)>::from(self)
    }
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateStatusParams {
    pub name: Option<String>,
    pub color: Option<backlog_domain_models::StatusColor>,
}

#[cfg(feature = "writable")]
impl From<&UpdateStatusParams> for Vec<(String, String)> {
    fn from(params: &UpdateStatusParams) -> Self {
        let mut seq = Vec::new();

        if let Some(name) = &params.name {
            seq.push(("name".to_string(), name.clone()));
        }

        if let Some(color) = &params.color {
            seq.push(("color".to_string(), color.as_hex().to_string()));
        }

        seq
    }
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteStatusParams {
    pub substitute_status_id: backlog_core::identifier::StatusId,
}

#[cfg(feature = "writable")]
impl From<&DeleteStatusParams> for Vec<(String, String)> {
    fn from(params: &DeleteStatusParams) -> Self {
        vec![(
            "substituteStatusId".to_string(),
            params.substitute_status_id.to_string(),
        )]
    }
}

#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UpdateStatusOrderParams {
    pub status_ids: Vec<backlog_core::identifier::StatusId>,
}

#[cfg(feature = "writable")]
impl From<&UpdateStatusOrderParams> for Vec<(String, String)> {
    fn from(params: &UpdateStatusOrderParams) -> Self {
        let mut seq = Vec::new();
        for status_id in &params.status_ids {
            seq.push(("statusId[]".to_string(), status_id.to_string()));
        }
        seq
    }
}
