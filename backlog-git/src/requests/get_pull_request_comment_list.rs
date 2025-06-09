use backlog_api_core::Error as ApiError;
use backlog_core::Error as CoreError;
use derive_builder::Builder;
use serde::Serialize;
use std::{fmt, str::FromStr};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PrCommentOrder {
    Asc,
    Desc,
}

impl fmt::Display for PrCommentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PrCommentOrder::Asc => write!(f, "asc"),
            PrCommentOrder::Desc => write!(f, "desc"),
        }
    }
}

impl FromStr for PrCommentOrder {
    type Err = CoreError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "asc" => Ok(PrCommentOrder::Asc),
            "desc" => Ok(PrCommentOrder::Desc),
            _ => Err(CoreError::InvalidParameter(format!(
                "Invalid pull request comment order: '{}'. Must be 'asc' or 'desc'.",
                s
            ))),
        }
    }
}

#[derive(Builder, Debug, Default, Serialize, Clone)]
#[builder(default, build_fn(error = "ApiError"))]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestCommentListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<PrCommentOrder>,
}
