#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
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
#[derive(Debug, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct AddIssueParams {
    #[builder(setter(into))]
    #[form(name = "projectId")]
    pub project_id: ProjectId,
    #[builder(setter(into))]
    pub summary: String,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentIssueId")]
    pub parent_issue_id: Option<IssueId>,
    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDate", date_format = "%Y-%m-%d")]
    pub start_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDate", date_format = "%Y-%m-%d")]
    pub due_date: Option<DateTime<Utc>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "estimatedHours")]
    pub estimated_hours: Option<f32>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "actualHours")]
    pub actual_hours: Option<f32>,
    #[builder(setter(into))]
    #[form(name = "issueTypeId")]
    pub issue_type_id: IssueTypeId,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "categoryId")]
    pub category_id: Option<Vec<CategoryId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "versionId")]
    pub version_id: Option<Vec<MilestoneId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "milestoneId")]
    pub milestone_id: Option<Vec<MilestoneId>>,
    #[builder(setter(into))]
    #[form(name = "priorityId")]
    pub priority_id: PriorityId,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "assigneeId")]
    pub assignee_id: Option<UserId>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "notifyUserId")]
    pub notify_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "attachmentId")]
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
        let params: Vec<(String, String)> = self.into();
        params
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_datetime_formatting_with_macros() {
        let params = AddIssueParamsBuilder::default()
            .project_id(ProjectId::new(1))
            .summary("Test Issue".to_string())
            .issue_type_id(IssueTypeId::new(1))
            .priority_id(PriorityId::new(1))
            .start_date(
                chrono::Utc
                    .with_ymd_and_hms(2024, 6, 24, 12, 30, 45)
                    .unwrap(),
            )
            .due_date(
                chrono::Utc
                    .with_ymd_and_hms(2024, 12, 31, 23, 59, 59)
                    .unwrap(),
            )
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // Check that dates are properly formatted
        let start_date_param = form_params.iter().find(|(key, _)| key == "startDate");
        assert!(start_date_param.is_some());
        assert_eq!(start_date_param.unwrap().1, "2024-06-24");

        let due_date_param = form_params.iter().find(|(key, _)| key == "dueDate");
        assert!(due_date_param.is_some());
        assert_eq!(due_date_param.unwrap().1, "2024-12-31");
    }
}
