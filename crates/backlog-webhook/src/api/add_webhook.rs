#[cfg(feature = "writable")]
use crate::models::Webhook;
#[cfg(feature = "writable")]
use backlog_api_core::{Error as ApiError, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use backlog_core::{ProjectIdOrKey, id::ActivityTypeId};
#[cfg(feature = "writable")]
use derive_builder::Builder;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type AddWebhookResponse = Webhook;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, Builder, ToFormParams)]
#[builder(build_fn(error = "ApiError"), setter(into))]
pub struct AddWebhookParams {
    #[form(skip)]
    pub project_id_or_key: ProjectIdOrKey,

    pub name: String,

    pub hook_url: String,

    #[builder(default, setter(strip_option))]
    pub description: Option<String>,

    #[builder(default, setter(strip_option))]
    pub all_event: Option<bool>,

    #[builder(default, setter(strip_option))]
    #[form(array, name = "activityTypeId")]
    pub activity_type_ids: Option<Vec<ActivityTypeId>>,
}

#[cfg(feature = "writable")]
impl IntoRequest for AddWebhookParams {
    fn method(&self) -> backlog_api_core::HttpMethod {
        backlog_api_core::HttpMethod::Post
    }

    fn path(&self) -> String {
        format!("/api/v2/projects/{}/webhooks", self.project_id_or_key)
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
