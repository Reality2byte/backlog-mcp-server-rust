#[cfg(feature = "writable")]
use crate::models::SpaceNotification;
#[cfg(feature = "writable")]
use backlog_api_core::{HttpMethod, IntoRequest};
#[cfg(feature = "writable")]
use backlog_api_macros::ToFormParams;
#[cfg(feature = "writable")]
use serde::Serialize;

#[cfg(feature = "writable")]
pub type UpdateSpaceNotificationResponse = SpaceNotification;

#[cfg(feature = "writable")]
#[derive(Debug, Clone, ToFormParams)]
pub struct UpdateSpaceNotificationParams {
    pub content: String,
}

#[cfg(feature = "writable")]
impl UpdateSpaceNotificationParams {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }
}

#[cfg(feature = "writable")]
impl IntoRequest for UpdateSpaceNotificationParams {
    fn method(&self) -> HttpMethod {
        HttpMethod::Put
    }

    fn path(&self) -> String {
        "/api/v2/space/notification".to_string()
    }

    fn to_form(&self) -> impl Serialize {
        let params: Vec<(String, String)> = self.into();
        params
    }
}
