use backlog_api_core::IntoRequest;
use serde::Serialize;

use crate::models::NotificationCount;

#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetNotificationCountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub already_read: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_already_read: Option<bool>,
}

impl GetNotificationCountParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_already_read(mut self, already_read: bool) -> Self {
        self.already_read = Some(already_read);
        self
    }

    pub fn with_resource_already_read(mut self, resource_already_read: bool) -> Self {
        self.resource_already_read = Some(resource_already_read);
        self
    }
}

impl IntoRequest for GetNotificationCountParams {
    fn path(&self) -> String {
        "/api/v2/notifications/count".to_string()
    }

    fn to_query(&self) -> impl Serialize {
        self
    }
}

pub type GetNotificationCountResponse = NotificationCount;
