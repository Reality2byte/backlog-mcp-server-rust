use backlog_api_core::{Error as ApiError, IntoDownloadRequest};
use backlog_core::{IssueIdOrKey, identifier::AttachmentId};
use derive_builder::Builder;

#[derive(Debug, Clone, Builder)]
#[builder(build_fn(error = "ApiError"))]
pub struct GetAttachmentFileParams {
    #[builder(setter(into))]
    pub issue_id_or_key: IssueIdOrKey,
    #[builder(setter(into))]
    pub attachment_id: AttachmentId,
}

impl GetAttachmentFileParams {
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

impl IntoDownloadRequest for GetAttachmentFileParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/issues/{}/attachments/{}",
            self.issue_id_or_key, self.attachment_id
        )
    }
}
