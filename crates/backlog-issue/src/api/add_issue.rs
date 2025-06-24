#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(all(feature = "writable", feature = "macros"))]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::identifier::{
    AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId, UserId,
};
#[cfg(feature = "writable")]
use chrono::{DateTime, Utc};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

/// Response type for adding a new issue
#[cfg(feature = "writable")]
pub type AddIssueResponse = Issue;

#[cfg(feature = "writable")]
#[derive(Debug, Builder)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
#[builder(build_fn(error = "ApiError"))]
pub struct AddIssueParams {
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(name = "projectId"))]
    pub project_id: ProjectId,
    #[builder(setter(into))]
    pub summary: String,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "parentIssueId"))]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(skip))]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(skip))]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "estimatedHours"))]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "actualHours"))]
    pub actual_hours: Option<f32>,
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(name = "issueTypeId"))]
    pub issue_type_id: IssueTypeId,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "categoryId"))]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "versionId"))]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "milestoneId"))]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(setter(into))]
    #[cfg_attr(feature = "macros", form(name = "priorityId"))]
    pub priority_id: PriorityId,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "assigneeId"))]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "notifyUserId"))]
    pub notify_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "attachmentId"))]
    pub attachment_id: Option<Vec<AttachmentId>>,
}

#[cfg(feature = "writable")]
impl IntoRequest for AddIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/issues".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        #[cfg(feature = "macros")]
        {
            self.to_form_params()
        }

        #[cfg(not(feature = "macros"))]
        {
            let mut params = vec![
                ("projectId".to_string(), self.project_id.to_string()),
                ("summary".to_string(), self.summary.clone()),
                ("issueTypeId".to_string(), self.issue_type_id.to_string()),
                ("priorityId".to_string(), self.priority_id.to_string()),
            ];

            if let Some(parent_issue_id) = &self.parent_issue_id {
                params.push(("parentIssueId".to_string(), parent_issue_id.to_string()));
            }

            if let Some(description) = &self.description {
                params.push(("description".to_string(), description.clone()));
            }

            if let Some(start_date) = &self.start_date {
                params.push((
                    "startDate".to_string(),
                    start_date.format("%Y-%m-%d").to_string(),
                ));
            }

            if let Some(due_date) = &self.due_date {
                params.push((
                    "dueDate".to_string(),
                    due_date.format("%Y-%m-%d").to_string(),
                ));
            }

            if let Some(estimated_hours) = self.estimated_hours {
                params.push(("estimatedHours".to_string(), estimated_hours.to_string()));
            }

            if let Some(actual_hours) = self.actual_hours {
                params.push(("actualHours".to_string(), actual_hours.to_string()));
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

            if let Some(assignee_id) = &self.assignee_id {
                params.push(("assigneeId".to_string(), assignee_id.to_string()));
            }

            if let Some(notify_user_ids) = &self.notify_user_id {
                for user_id in notify_user_ids {
                    params.push(("notifyUserId[]".to_string(), user_id.to_string()));
                }
            }

            if let Some(attachment_ids) = &self.attachment_id {
                for attachment_id in attachment_ids {
                    params.push(("attachmentId[]".to_string(), attachment_id.to_string()));
                }
            }

            params
        }
    }
}

// Extension method to handle date fields when using macros
#[cfg(all(feature = "writable", feature = "macros"))]
impl AddIssueParams {
    fn to_form_params(&self) -> Vec<(String, String)> {
        let mut params: Vec<(String, String)> = self.into();

        // Handle date fields with special formatting
        if let Some(start_date) = &self.start_date {
            params.push((
                "startDate".to_string(),
                start_date.format("%Y-%m-%d").to_string(),
            ));
        }

        if let Some(due_date) = &self.due_date {
            params.push((
                "dueDate".to_string(),
                due_date.format("%Y-%m-%d").to_string(),
            ));
        }

        params
    }
}
