#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::IssueIdOrKey;
#[cfg(feature = "writable")]
use backlog_core::identifier::SharedFileId;

#[cfg(feature = "writable")]
use crate::models::SharedFile;

/// Response type for unlink shared file operations.
#[cfg(feature = "writable")]
pub type UnlinkSharedFileResponse = SharedFile;

/// Parameters for unlinking a shared file from an issue.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct UnlinkSharedFileParams {
    pub issue_id_or_key: IssueIdOrKey,
    pub shared_file_id: SharedFileId,
}

#[cfg(feature = "writable")]
impl UnlinkSharedFileParams {
    /// Creates new parameters for unlinking a shared file.
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        shared_file_id: impl Into<SharedFileId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            shared_file_id: shared_file_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UnlinkSharedFileParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/sharedFiles/{}",
            self.issue_id_or_key, self.shared_file_id
        )
    }
}
