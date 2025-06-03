use backlog_api_core::Error as ApiError;
use backlog_core::identifier::{
    AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId,
    UserId,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;

#[derive(serde::Serialize, Debug, Builder)]
#[serde(rename_all = "camelCase")]
#[builder(build_fn(error = "ApiError"))]
pub struct AddIssueParams {
    #[builder(setter(into))]
    pub project_id: ProjectId,
    #[builder(setter(into))]
    pub summary: String,
    #[builder(default, setter(into, strip_option))]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    pub actual_hours: Option<f32>,
    #[builder(setter(into))]
    pub issue_type_id: IssueTypeId,
    #[builder(default, setter(into, strip_option))]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(setter(into))]
    pub priority_id: PriorityId,
    #[builder(default, setter(into, strip_option))]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    pub notify_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    pub attachment_id: Option<Vec<AttachmentId>>,
}
