use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
use backlog_core::identifier::{
    AttachmentId, CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId, UserId,
};
use chrono::{DateTime, Utc};
use derive_builder::Builder;
use serde::Serialize;

#[derive(Debug, Builder)]
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

impl IntoRequest for AddIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Post
    }

    fn path(&self) -> String {
        "/api/v2/issues".to_string()
    }

    fn to_form(&self) -> impl Serialize {
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
