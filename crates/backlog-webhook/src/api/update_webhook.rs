#[cfg(feature = "writable")]
use crate::models::Webhook;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{
    ProjectIdOrKey,
    id::{ActivityTypeId, WebhookId},
};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateWebhookResponse = Webhook;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"))]
pub struct UpdateWebhookParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    #[form(skip)]
    pub webhook_id: WebhookId,

    #[builder(default, setter(into, strip_option))]
    pub name: Option<String>,

    #[builder(default, setter(into, strip_option))]
    pub description: Option<String>,

    #[builder(default, setter(into, strip_option))]
    pub hook_url: Option<String>,

    #[builder(default, setter(strip_option))]
    pub all_event: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[form(array, name = "activityTypeId")]
    pub activity_type_ids: Option<Vec<ActivityTypeId>>,
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateWebhookParams {
    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Patch
    }

    fn path(&self) -> String {
        format!(
            "/api/v2/projects/{}/webhooks/{}",
            self.project_id_or_key, self.webhook_id
        )
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
