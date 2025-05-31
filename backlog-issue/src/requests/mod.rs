use crate::models::parent_child::ParentChildCondition;
use backlog_api_core::Error as ApiError;
use backlog_core::identifier::{
    AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId,
    ResolutionId, StatusId, UserId,
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

#[derive(Debug, Builder)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetIssueListParams {
    #[builder(default, setter(into, strip_option))]
    pub project_id: Option<Vec<ProjectId>>,
    #[builder(default, setter(into, strip_option))]
    pub issue_type_id: Option<Vec<IssueTypeId>>,
    #[builder(default, setter(into, strip_option))]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    pub status_id: Option<Vec<StatusId>>, // Changed from String to StatusId
    #[builder(default, setter(into, strip_option))]
    pub priority_id: Option<Vec<PriorityId>>,
    #[builder(default, setter(into, strip_option))]
    pub assignee_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    pub created_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    pub resolution_id: Option<Vec<ResolutionId>>,
    #[builder(default, setter(into, strip_option))]
    pub parent_child_condition: Option<ParentChildCondition>,
    #[builder(default, setter(into, strip_option))]
    pub attachment: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    pub shared_file: Option<bool>, // Added
    #[builder(default, setter(into, strip_option))]
    pub sort: Option<String>, // Added (e.g., "issueType", "created")
    #[builder(default, setter(into, strip_option))]
    pub order: Option<String>, // Added ("asc" or "desc")
    #[builder(default, setter(into, strip_option))]
    pub offset: Option<u32>, // Added
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u32>, // Added (1-100)
    #[builder(default, setter(into, strip_option))]
    pub created_since: Option<String>, // Added (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    pub created_until: Option<String>, // Added (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    pub updated_since: Option<String>, // Added (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    pub updated_until: Option<String>, // Added (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    pub parent_issue_id: Option<Vec<IssueId>>, // Added (Note: different from single parentIssueId in Add/Update)
    #[builder(default, setter(into, strip_option))]
    pub keyword: Option<String>, // Added
    #[builder(default, setter(into, strip_option))]
    pub id: Option<Vec<IssueId>>, // Added for id[] parameter
}

// Convert GetIssueListParams to vector of pairs because
// RequestBuilder.query doesn't support serialization of vector type.
impl From<GetIssueListParams> for Vec<(String, String)> {
    fn from(params: GetIssueListParams) -> Self {
        let mut seq = Vec::new();

        macro_rules! push_val {
            ($field:expr, $key:expr) => {
                if let Some(value) = $field {
                    seq.push(($key.to_string(), value.to_string()));
                }
            };
        }

        macro_rules! push_vec {
            ($field:expr, $key:expr) => {
                if let Some(values) = $field {
                    values
                        .iter()
                        .for_each(|v| seq.push(($key.to_string(), v.to_string())));
                }
            };
        }

        push_vec!(params.project_id, "projectId[]");
        push_vec!(params.issue_type_id, "issueTypeId[]");
        push_vec!(params.category_id, "categoryId[]");
        push_vec!(params.version_id, "versionId[]");
        push_vec!(params.milestone_id, "milestoneId[]");
        push_vec!(params.status_id, "statusId[]");
        push_vec!(params.priority_id, "priorityId[]");
        push_vec!(params.assignee_id, "assigneeId[]");
        push_vec!(params.created_user_id, "createdUserId[]");
        push_vec!(params.resolution_id, "resolutionId[]");
        push_vec!(params.parent_issue_id, "parentIssueId[]"); // Added
        push_vec!(params.id, "id[]"); // Added

        if let Some(parent_child_condition) = params.parent_child_condition {
            seq.push((
                "parentChild".to_string(), // API doc uses "parentChild" not "parentChildCondition"
                (parent_child_condition as u8).to_string(),
            ));
        }

        push_val!(params.attachment, "attachment");
        push_val!(params.shared_file, "sharedFile"); // Added
        push_val!(params.sort, "sort"); // Added
        push_val!(params.order, "order"); // Added
        push_val!(params.offset, "offset"); // Added
        push_val!(params.count, "count"); // Added
        push_val!(params.created_since, "createdSince"); // Added
        push_val!(params.created_until, "createdUntil"); // Added
        push_val!(params.updated_since, "updatedSince"); // Added
        push_val!(params.updated_until, "updatedUntil"); // Added
        push_val!(params.keyword, "keyword"); // Added

        // Custom fields would be handled here if implemented
        // e.g., params.custom_fields.iter().for_each(|(k,v)| seq.push((k.clone(), v.clone())));

        seq
    }
}

pub type CountIssueParams = GetIssueListParams; // This might need to be a subset if count doesn't support all list params
pub type CountIssueParamsBuilder = GetIssueListParamsBuilder;

#[derive(serde::Serialize, Debug, Builder, Default)]
#[builder(default, build_fn(error = "ApiError"))]
#[serde(rename_all = "camelCase")]
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
