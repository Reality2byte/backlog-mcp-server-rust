#[cfg(feature = "writable")]
use crate::models::Webhook;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, id::WebhookId};

/// Response type for deleting a webhook
#[cfg(feature = "writable")]
pub type DeleteWebhookResponse = Webhook;

/// Parameters for deleting a specific webhook.
/// Corresponds to `DELETE /api/v2/projects/:projectIdOrKey/webhooks/:webhookId`.
#[cfg(feature = "writable")]
#[derive(Debug, Clone)]
pub struct DeleteWebhookParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub webhook_id: WebhookId,
}

#[cfg(feature = "writable")]
impl DeleteWebhookParams {
    pub fn new(
        project_id_or_key: impl Into<ProjectIdOrKey>,
        webhook_id: impl Into<WebhookId>,
    ) -> Self {
        Self {
            project_id_or_key: project_id_or_key.into(),
            webhook_id: webhook_id.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for DeleteWebhookParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Delete
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/webhooks/{}",
            self.project_id_or_key, self.webhook_id
        )
    }
}
