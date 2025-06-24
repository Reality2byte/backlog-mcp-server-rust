#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(all(feature = "writable", feature = "macros"))]
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
#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
#[builder(setter(strip_option, into))]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdateIssueParams {
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(skip))]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(default, setter(into, strip_option))]
    pub summary: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "parentIssueId"))]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "startDate"))]
    pub start_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "dueDate"))]
    pub due_date: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "estimatedHours"))]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "actualHours"))]
    pub actual_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "issueTypeId"))]
    pub issue_type_id: Option<IssueTypeId>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "categoryId"))]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "versionId"))]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "milestoneId"))]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "priorityId"))]
    pub priority_id: Option<PriorityId>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "assigneeId"))]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "notifiedUserId"))]
    pub notified_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "attachmentId"))]
    pub attachment_id: Option<Vec<AttachmentId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "statusId"))]
    pub status_id: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "resolutionId"))]
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
        #[cfg(feature = "macros")]
        {
            let params: Vec<(String, String)> = self.into();
            params
        }

        #[cfg(not(feature = "macros"))]
        {
            let mut params = Vec::new();

            if let Some(summary) = &self.summary {
                params.push(("summary".to_string(), summary.clone()));
            }

            if let Some(parent_issue_id) = &self.parent_issue_id {
                params.push(("parentIssueId".to_string(), parent_issue_id.to_string()));
            }

            if let Some(description) = &self.description {
                params.push(("description".to_string(), description.clone()));
            }

            if let Some(start_date) = &self.start_date {
                params.push(("startDate".to_string(), start_date.to_string()));
            }

            if let Some(due_date) = &self.due_date {
                params.push(("dueDate".to_string(), due_date.to_string()));
            }

            if let Some(estimated_hours) = self.estimated_hours {
                params.push(("estimatedHours".to_string(), estimated_hours.to_string()));
            }

            if let Some(actual_hours) = self.actual_hours {
                params.push(("actualHours".to_string(), actual_hours.to_string()));
            }

            if let Some(issue_type_id) = &self.issue_type_id {
                params.push(("issueTypeId".to_string(), issue_type_id.to_string()));
            }

            if let Some(category_ids) = &self.category_id {
                for category_id in category_ids {
                    params.push(("categoryId[]".to_string(), category_id.to_string()));
                }
            }

            if let Some(version_ids) = &self.version_id {
                for version_id in version_ids {
                    params.push(("versionId[]".to_string(), version_id.to_string()));
                }
            }

            if let Some(milestone_ids) = &self.milestone_id {
                for milestone_id in milestone_ids {
                    params.push(("milestoneId[]".to_string(), milestone_id.to_string()));
                }
            }

            if let Some(priority_id) = &self.priority_id {
                params.push(("priorityId".to_string(), priority_id.to_string()));
            }

            if let Some(assignee_id) = &self.assignee_id {
                params.push(("assigneeId".to_string(), assignee_id.to_string()));
            }

            if let Some(notified_user_ids) = &self.notified_user_id {
                for user_id in notified_user_ids {
                    params.push(("notifiedUserId[]".to_string(), user_id.to_string()));
                }
            }

            if let Some(attachment_ids) = &self.attachment_id {
                for attachment_id in attachment_ids {
                    params.push(("attachmentId[]".to_string(), attachment_id.to_string()));
                }
            }

            if let Some(status_id) = &self.status_id {
                params.push(("statusId".to_string(), status_id.clone()));
            }

            if let Some(resolution_id) = &self.resolution_id {
                params.push(("resolutionId".to_string(), resolution_id.to_string()));
            }

            if let Some(comment) = &self.comment {
                params.push(("comment".to_string(), comment.clone()));
            }

            params
        }
    }
}
