#[cfg(feature = "writable")]
use crate::models::Issue;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::IssueKey;

/// Response type for deleting an issue
#[cfg(feature = "writable")]
pub type DeleteIssueResponse = Issue;

/// Parameters for deleting a specific issue.
/// Corresponds to `DELETE /api/v2/issues/:issueKey`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone, PartialEq)]
pub struct DeleteIssueParams {
    pub issue_key: IssueKey,
}

#[cfg(feature = "writable")]
impl DeleteIssueParams {
    pub fn new(issue_key: impl Into<IssueKey>) -> Self {
        Self {
            issue_key: issue_key.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}", self.issue_key)
    }
}

#[cfg(all(test, feature = "writable"))]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_delete_issue_params_new() {
        let issue_key = IssueKey::from_str("TEST-123").unwrap();
        let params = DeleteIssueParams::new(issue_key.clone());
        assert_eq!(params.issue_key, issue_key);
    }

    #[test]
    fn test_delete_issue_params_into_request() {
        let issue_key = IssueKey::from_str("TEST-123").unwrap();
        let params = DeleteIssueParams::new(issue_key);

        assert_eq!(params.method(), HttpMethod::Delete);
        assert_eq!(params.path(), "/api/v2/issues/TEST-123");
    }
}
