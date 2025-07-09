use crate::api::get_webhook::{GetWebhookParams, GetWebhookResponse};
use crate::api::get_webhook_list::{GetWebhookListParams, GetWebhookListResponse};
use backlog_api_core::Result;
use backlog_core::{ProjectIdOrKey, id::WebhookId};
use client::Client;

pub struct WebhookApi(Client);

impl WebhookApi {
    pub fn new(client: Client) -> Self {
        Self(client)
    }

    /// Get list of webhooks in a project.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/webhooks`.
    pub async fn get_webhook_list(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
    ) -> Result<GetWebhookListResponse> {
        let params = GetWebhookListParams {
            project_id_or_key: project_id_or_key.into(),
        };
        self.0.execute(params).await
    }

    /// Get webhook information.
    /// Corresponds to `GET /api/v2/projects/:projectIdOrKey/webhooks/:webhookId`.
    pub async fn get_webhook(
        &self,
        project_id_or_key: impl Into<ProjectIdOrKey>,
        webhook_id: impl Into<WebhookId>,
    ) -> Result<GetWebhookResponse> {
        let params = GetWebhookParams {
            project_id_or_key: project_id_or_key.into(),
            webhook_id: webhook_id.into(),
        };
        self.0.execute(params).await
    }
}
