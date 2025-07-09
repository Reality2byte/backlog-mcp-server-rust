use crate::models::Webhook;
use backlog_api_core::IntoRequest;
use backlog_core::{ProjectIdOrKey, id::WebhookId};
use serde::Serialize;

pub type GetWebhookResponse = Webhook;

#[derive(Debug, Clone)]
pub struct GetWebhookParams {
    pub project_id_or_key: ProjectIdOrKey,
    pub webhook_id: WebhookId,
}

impl IntoRequest for GetWebhookParams {
    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/webhooks/{}",
            self.project_id_or_key, self.webhook_id
        )
    }

    fn to_query(&self) -> impl Serialize {
        Vec::<(String, String)>::new()
    }
}
