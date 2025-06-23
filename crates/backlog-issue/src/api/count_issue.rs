// Re-export from get_issue_list since CountIssue uses the same parameters
pub use super::get_issue_list::{
    GetIssueListParams as CountIssueParams, GetIssueListParamsBuilder as CountIssueParamsBuilder,
};
use serde::Deserialize;

/// Response type for counting issues
#[derive(Debug, Deserialize)]
pub struct CountIssueResponse {
    pub count: u32,
}
