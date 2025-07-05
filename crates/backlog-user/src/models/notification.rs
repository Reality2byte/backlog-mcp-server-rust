use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct NotificationCount {
    pub count: u32,
}
