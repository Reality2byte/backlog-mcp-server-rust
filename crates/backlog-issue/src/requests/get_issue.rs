use backlog_api_core::{HttpMethod, IntoRequest};
use backlog_core::IssueIdOrKey;
use serde::Serialize;

/// Parameters for getting a specific issue.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetIssueParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl GetIssueParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetIssueParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!("/api/v2/issues/{}", self.issue_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use backlog_core::IssueKey;
    use std::str::FromStr;

    #[test]
    fn test_get_issue_params_new() {
        let issue_id_or_key = IssueIdOrKey::Key(IssueKey::from_str("TEST-123").unwrap());
        let params = GetIssueParams::new(issue_id_or_key.clone());
        assert_eq!(params.issue_id_or_key, issue_id_or_key);
    }

    #[test]
    fn test_get_issue_params_into_request() {
        let issue_id_or_key = IssueIdOrKey::Key(IssueKey::from_str("TEST-123").unwrap());
        let params = GetIssueParams::new(issue_id_or_key);

        assert_eq!(params.method(), HttpMethod::Get);
        assert_eq!(params.path(), "/api/v2/issues/TEST-123");
    }
}
