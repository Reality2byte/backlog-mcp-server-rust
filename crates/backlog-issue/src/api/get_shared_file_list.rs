use crate::models::SharedFile;
use backlog_api_core::IntoRequest;
use backlog_core::IssueIdOrKey;

/// Response type for getting a list of shared files linked to an issue
pub type GetSharedFileListResponse = Vec<SharedFile>;

/// Parameters for getting shared file list for a specific issue.
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/sharedFiles`.
#[derive(Debug, Clone, PartialEq)]
pub struct GetSharedFileListParams {
    pub issue_id_or_key: IssueIdOrKey,
}

impl GetSharedFileListParams {
    pub fn new(issue_id_or_key: impl Into<IssueIdOrKey>) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
        }
    }
}

impl IntoRequest for GetSharedFileListParams {
    fn path(&self) -> String {
        format!("/api/v2/issues/{}/sharedFiles", self.issue_id_or_key)
    }
}
