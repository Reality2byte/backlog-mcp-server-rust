#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    ApiDate, IssueIdOrKey,
    identifier::{
        AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ResolutionId,
        UserId,
    },
};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for updating an issue
#[cfg(feature = "writable")]
pub type UpdateIssueResponse = Issue;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(setter(strip_option, into))]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdateIssueParams {
    #[builder(setter(into))]
    #[form(skip)]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(default, setter(into, strip_option))]
    pub summary: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentIssueId")]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDate")]
    pub start_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDate")]
    pub due_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "estimatedHours")]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "actualHours")]
    pub actual_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "issueTypeId")]
    pub issue_type_id: Option<IssueTypeId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "categoryId")]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "versionId")]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "milestoneId")]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "priorityId")]
    pub priority_id: Option<PriorityId>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "assigneeId")]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "notifiedUserId")]
    pub notified_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "attachmentId")]
    pub attachment_id: Option<Vec<AttachmentId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "statusId")]
    pub status_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "resolutionId")]
    pub resolution_id: Option<ResolutionId>,
    #[builder(default, setter(into, strip_option))]
    pub comment: Option<String>,
    // Custom fields are omitted for now due to their dynamic nature.
    // customField_{id}
    // customField_{id}_otherValue
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}", self.issue_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
