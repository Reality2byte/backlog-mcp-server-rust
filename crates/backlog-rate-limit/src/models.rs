use backlog_api_core::ApiRateLimit;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitResponse {
    #[serde(rename = "rateLimit")]
    pub rate_limit: RateLimitInfo,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RateLimitInfo {
    pub read: ApiRateLimit,
    pub update: ApiRateLimit,
    pub search: ApiRateLimit,
    pub icon: ApiRateLimit,
}

pub type GetRateLimitResponse = RateLimitResponse;
