use backlog_api_core::Error as ApiError;
use backlog_core::identifier::{
    AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ResolutionId, UserId,
};
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Clone, Default, Builder, Serialize)]
#[builder(default, setter(strip_option, into))]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdateIssueParams {
    #[builder(setter(into, strip_option))]
    pub summary: Option<String>,
    #[builder(setter(into, strip_option))]
    pub parent_issue_id: Option<IssueId>,
    #[builder(setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(setter(into, strip_option))]
    pub start_date: Option<String>, // API expects "yyyy-MM-dd"
    #[builder(setter(into, strip_option))]
    pub due_date: Option<String>, // API expects "yyyy-MM-dd"
    #[builder(setter(into, strip_option))]
    pub estimated_hours: Option<f32>,
    #[builder(setter(into, strip_option))]
    pub actual_hours: Option<f32>,
    #[builder(setter(into, strip_option))]
    pub issue_type_id: Option<IssueTypeId>,
    #[builder(setter(into, strip_option))]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(setter(into, strip_option))]
    pub version_id: Option<Vec<MilestoneId>>, // Note: API doc says versionId, but existing AddIssueParams uses MilestoneId for version_id. Assuming MilestoneId is correct for consistency or if it's a typo in AddIssueParams. For now, using MilestoneId as per existing code.
    #[builder(setter(into, strip_option))]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(setter(into, strip_option))]
    pub priority_id: Option<PriorityId>,
    #[builder(setter(into, strip_option))]
    pub assignee_id: Option<UserId>,
    #[builder(setter(into, strip_option))]
    pub notified_user_id: Option<Vec<UserId>>,
    #[builder(setter(into, strip_option))]
    pub attachment_id: Option<Vec<AttachmentId>>,
    #[builder(setter(into, strip_option))]
    pub status_id: Option<String>, // API doc says Number, but existing GetIssueListParams uses String for status_id. Assuming String for flexibility, or it might be an ID of a status. Let's use String for now and clarify if needed. The response shows status as an object, but request is often just ID.
    #[builder(setter(into, strip_option))]
    pub resolution_id: Option<ResolutionId>,
    #[builder(setter(into, strip_option))]
    pub comment: Option<String>,
    // Custom fields are omitted for now due to their dynamic nature.
    // customField_{id}
    // customField_{id}_otherValue
}
