use crate::models::Webhook;
use backlog_api_core::IntoRequest;
use backlog_core::ProjectIdOrKey;
use serde::Serialize;

pub type GetWebhookListResponse = Vec<Webhook>;

#[derive(Debug, Clone)]
pub struct GetWebhookListParams {
    pub project_id_or_key: ProjectIdOrKey,
}

impl IntoRequest for GetWebhookListParams {
    fn path(&self) -> String {
        format!("/api/v2/projects/{}/webhooks", self.project_id_or_key)
    }

    fn to_query(&self) -> impl Serialize {
        std::collections::HashMap::<String, String>::new()
    }
}
