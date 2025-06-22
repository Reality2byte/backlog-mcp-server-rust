use backlog_api_core::{HttpMethod, IntoDownloadRequest, IntoRequest};
use backlog_core::{
    IssueIdOrKey,
    identifier::{AttachmentId, Identifier},
};
use derive_builder::Builder;
use serde::Serialize;

/// Parameters for downloading an issue attachment file.
///
/// Corresponds to `GET /api/v2/issues/:issueIdOrKey/attachments/:attachmentId`.
#[derive(Builder, Debug, Clone)]
#[builder(build_fn(error = "backlog_api_core::Error"))]
pub struct GetAttachmentFileParams {
    /// The issue ID or key.
    pub issue_id_or_key: IssueIdOrKey,
    /// The attachment ID.
    pub attachment_id: AttachmentId,
}

impl GetAttachmentFileParams {
    /// Creates a new instance with the required parameters.
    pub fn new(
        issue_id_or_key: impl Into<IssueIdOrKey>,
        attachment_id: impl Into<AttachmentId>,
    ) -> Self {
        Self {
            issue_id_or_key: issue_id_or_key.into(),
            attachment_id: attachment_id.into(),
        }
    }
}

impl IntoRequest for GetAttachmentFileParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/attachments/{}",
            self.issue_id_or_key,
            self.attachment_id.value()
        )
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}

impl IntoDownloadRequest for GetAttachmentFileParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/attachments/{}",
            self.issue_id_or_key,
            self.attachment_id.value()
        )
    }
}
