use crate::models::{Issue, ParentChildCondition};
use backlog_api_core::{Error as ApiError, IntoRequest};
use backlog_api_macros::ToFormParams;
use backlog_core::ApiDate;
use backlog_core::identifier::{
    CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
    UserId,
};
use derive_builder::Builder;
use serde::Serialize;

/// Response type for getting a list of issues
pub type GetIssueListResponse = Vec<Issue>;

#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetIssueListParams {
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "projectId")]
    pub project_id: Option<Vec<ProjectId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "issueTypeId")]
    pub issue_type_id: Option<Vec<IssueTypeId>>,
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
    #[form(array, name = "statusId")]
    pub status_id: Option<Vec<StatusId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "priorityId")]
    pub priority_id: Option<Vec<PriorityId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "assigneeId")]
    pub assignee_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "createdUserId")]
    pub created_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "resolutionId")]
    pub resolution_id: Option<Vec<ResolutionId>>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "parentChild")]
    pub parent_child_condition: Option<ParentChildCondition>,
    #[builder(default, setter(into, strip_option))]
    pub attachment: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "sharedFile")]
    pub shared_file: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    pub sort: Option<String>, // (e.g., "issueType", "created")
    #[builder(default, setter(into, strip_option))]
    pub order: Option<String>, // ("asc" or "desc")
    #[builder(default, setter(into, strip_option))]
    pub offset: Option<u32>,
    #[builder(default, setter(into, strip_option))]
    pub count: Option<u32>, // (1-100)
    #[builder(default, setter(into, strip_option))]
    #[form(name = "createdSince")]
    pub created_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "createdUntil")]
    pub created_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "updatedSince")]
    pub updated_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "updatedUntil")]
    pub updated_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(array, name = "parentIssueId")]
    pub parent_issue_id: Option<Vec<IssueId>>, // (Note: different from single parentIssueId in Add/Update)
    #[builder(default, setter(into, strip_option))]
    pub keyword: Option<String>, // (e.g., "bug", "feature")
    #[builder(default, setter(into, strip_option))]
    #[form(array)]
    pub id: Option<Vec<IssueId>>, // for id[] parameter
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDateSince")]
    pub start_date_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "startDateUntil")]
    pub start_date_until: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDateSince")]
    pub due_date_since: Option<ApiDate>,
    #[builder(default, setter(into, strip_option))]
    #[form(name = "dueDateUntil")]
    pub due_date_until: Option<ApiDate>,
}

impl IntoRequest for GetIssueListParams {
    fn path(&self) -> String {
        "/api/v2/issues".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}

// Convert GetIssueListParams to vector of pairs because
// RequestBuilder.query doesn't support serialization of vector type.
// Support both owned and borrowed values
impl From<GetIssueListParams> for Vec<(String, String)> {
    fn from(params: GetIssueListParams) -> Self {
        (&params).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parent_child_condition_serialization() {
        let params = GetIssueListParamsBuilder::default()
            .parent_child_condition(ParentChildCondition::ChildIssue)
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // Check that parent_child_condition is properly serialized
        let parent_child_param = form_params.iter().find(|(key, _)| key == "parentChild");

        assert!(parent_child_param.is_some());
        assert_eq!(parent_child_param.unwrap().1, "2");
    }

    #[test]
    fn test_parent_child_condition_all_values() {
        for condition in ParentChildCondition::all() {
            let params = GetIssueListParamsBuilder::default()
                .parent_child_condition(*condition)
                .build()
                .unwrap();

            let form_params: Vec<(String, String)> = (&params).into();
            let parent_child_param = form_params.iter().find(|(key, _)| key == "parentChild");

            assert!(parent_child_param.is_some());
            assert_eq!(
                parent_child_param.unwrap().1,
                (*condition as u8).to_string()
            );
        }
    }

    #[test]
    fn test_date_range_parameters() {
        use chrono::{TimeZone, Utc};

        let start_date = ApiDate::from(Utc.with_ymd_and_hms(2024, 1, 1, 0, 0, 0).unwrap());
        let end_date = ApiDate::from(Utc.with_ymd_and_hms(2024, 12, 31, 0, 0, 0).unwrap());

        let params = GetIssueListParamsBuilder::default()
            .start_date_since(start_date)
            .start_date_until(end_date)
            .due_date_since(start_date)
            .due_date_until(end_date)
            .build()
            .unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        let start_date_since = form_params.iter().find(|(key, _)| key == "startDateSince");
        let start_date_until = form_params.iter().find(|(key, _)| key == "startDateUntil");
        let due_date_since = form_params.iter().find(|(key, _)| key == "dueDateSince");
        let due_date_until = form_params.iter().find(|(key, _)| key == "dueDateUntil");

        assert!(start_date_since.is_some());
        assert_eq!(start_date_since.unwrap().1, "2024-01-01");

        assert!(start_date_until.is_some());
        assert_eq!(start_date_until.unwrap().1, "2024-12-31");

        assert!(due_date_since.is_some());
        assert_eq!(due_date_since.unwrap().1, "2024-01-01");

        assert!(due_date_until.is_some());
        assert_eq!(due_date_until.unwrap().1, "2024-12-31");
    }

    #[test]
    fn test_optional_date_range_parameters() {
        let params = GetIssueListParamsBuilder::default().build().unwrap();

        let form_params: Vec<(String, String)> = (&params).into();

        // Check that date parameters are not included when None
        let has_start_date_since = form_params.iter().any(|(key, _)| key == "startDateSince");
        let has_start_date_until = form_params.iter().any(|(key, _)| key == "startDateUntil");
        let has_due_date_since = form_params.iter().any(|(key, _)| key == "dueDateSince");
        let has_due_date_until = form_params.iter().any(|(key, _)| key == "dueDateUntil");

        assert!(!has_start_date_since);
        assert!(!has_start_date_until);
        assert!(!has_due_date_since);
        assert!(!has_due_date_until);
    }
}
