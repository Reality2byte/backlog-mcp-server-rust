use backlog_api_core::Error;
use derive_builder::Builder;
use serde::Serialize;

#[derive(Builder, Debug, Default, Serialize, Clone)]
#[builder(default, build_fn(error = "Error"))]
#[serde(rename_all = "camelCase")]
pub struct GetPullRequestCommentListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
}
