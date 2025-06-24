use crate::models::{Issue, ParentChildCondition};
use backlog_api_core::{Error as ApiError, IntoRequest};
#[cfg(feature = "macros")]
use backlog_api_macros::ToFormParams;
use backlog_core::identifier::{
    CategoryId, IssueId, IssueTypeId, MilestoneId, PriorityId, ProjectId, ResolutionId, StatusId,
    UserId,
};
use derive_builder::Builder;
use serde::Serialize;

/// Response type for getting a list of issues
pub type GetIssueListResponse = Vec<Issue>;

#[derive(Debug, Clone, Builder)]
#[cfg_attr(feature = "macros", derive(ToFormParams))]
#[builder(build_fn(error = "ApiError"))]
pub struct GetIssueListParams {
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "projectId"))]
    pub project_id: Option<Vec<ProjectId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "issueTypeId"))]
    pub issue_type_id: Option<Vec<IssueTypeId>>,
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
    #[cfg_attr(feature = "macros", form(array, name = "statusId"))]
    pub status_id: Option<Vec<StatusId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "priorityId"))]
    pub priority_id: Option<Vec<PriorityId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "assigneeId"))]
    pub assignee_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "createdUserId"))]
    pub created_user_id: Option<Vec<UserId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "resolutionId"))]
    pub resolution_id: Option<Vec<ResolutionId>>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "parentChild"))]
    pub parent_child_condition: Option<ParentChildCondition>,
    #[builder(default, setter(into, strip_option))]
    pub attachment: Option<bool>,
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "sharedFile"))]
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
    #[cfg_attr(feature = "macros", form(name = "createdSince"))]
    pub created_since: Option<String>, // (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "createdUntil"))]
    pub created_until: Option<String>, // (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "updatedSince"))]
    pub updated_since: Option<String>, // (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(name = "updatedUntil"))]
    pub updated_until: Option<String>, // (yyyy-MM-dd)
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array, name = "parentIssueId"))]
    pub parent_issue_id: Option<Vec<IssueId>>, // (Note: different from single parentIssueId in Add/Update)
    #[builder(default, setter(into, strip_option))]
    pub keyword: Option<String>, // (e.g., "bug", "feature")
    #[builder(default, setter(into, strip_option))]
    #[cfg_attr(feature = "macros", form(array))]
    pub id: Option<Vec<IssueId>>, // for id[] parameter
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

#[cfg(not(feature = "macros"))]
impl From<&GetIssueListParams> for Vec<(String, String)> {
    fn from(params: &GetIssueListParams) -> Self {
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
                if let Some(values) = &$field {
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
        push_vec!(params.parent_issue_id, "parentIssueId[]");
        push_vec!(params.id, "id[]");

        if let Some(parent_child_condition) = &params.parent_child_condition {
            seq.push((
                "parentChild".to_string(), // API doc uses "parentChild" not "parentChildCondition"
                (parent_child_condition.clone() as u8).to_string(),
            ));
        }

        push_val!(&params.attachment, "attachment");
        push_val!(&params.shared_file, "sharedFile");
        push_val!(&params.sort, "sort");
        push_val!(&params.order, "order");
        push_val!(&params.offset, "offset");
        push_val!(&params.count, "count");
        push_val!(&params.created_since, "createdSince");
        push_val!(&params.created_until, "createdUntil");
        push_val!(&params.updated_since, "updatedSince");
        push_val!(&params.updated_until, "updatedUntil");
        push_val!(&params.keyword, "keyword");
        push_val!(&params.parent_child_condition, "parentChild");

        // Custom fields would be handled here if implemented
        // e.g., params.custom_fields.iter().for_each(|(k,v)| seq.push((k.clone(), v.clone())));

        seq
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
}
