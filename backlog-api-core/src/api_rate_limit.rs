use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct ApiRateLimit {
    pub limit: i32,
    pub remaining: i32,
    pub reset: i32,
}
