use backlog_api_core::Error as ApiError; // Added this import
use derive_builder::Builder;
use serde::Serialize;
use std::fmt;

/// Specifies the sort order for listing comments.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CommentOrder {
    /// Sorts comments in ascending order (oldest first).
    Asc,
    /// Sorts comments in descending order (newest first).
    Desc,
}

impl fmt::Display for CommentOrder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommentOrder::Asc => write!(f, "asc"),
            CommentOrder::Desc => write!(f, "desc"),
        }
    }
}

/// Parameters for [IssueApi::get_comment_list](crate::api::IssueApi::get_comment_list).
///
/// Allows filtering and pagination when retrieving comments for an issue.
/// Use the associated builder `GetCommentListParamsBuilder` to construct an instance.
///
/// # Example
///
/// ```
/// use backlog_issue::requests::get_comment_list::{GetCommentListParamsBuilder, CommentOrder};
///
/// let params = GetCommentListParamsBuilder::default()
///     .min_id(100u64)
///     .count(20u8)
///     .order(CommentOrder::Asc)
///     .build()
///     .unwrap();
/// ```
#[derive(Debug, Clone, Default, Builder)]
#[builder(default, setter(strip_option, into), build_fn(error = "ApiError"))] // Added build_fn
pub struct GetCommentListParams {
    /// The minimum comment ID to include in the results.
    #[builder(default)]
    min_id: Option<u64>,
    /// The maximum comment ID to include in the results.
    #[builder(default)]
    max_id: Option<u64>,
    /// The number of comments to retrieve (default: 20, max: 100).
    #[builder(default)]
    count: Option<u8>,
    /// The sort order for the comments.
    #[builder(default)]
    order: Option<CommentOrder>,
}

impl GetCommentListParams {
    /// Converts the parameters to a vector of (key, value) string pairs
    /// suitable for use as URL query parameters.
    pub fn to_query_params(&self) -> Vec<(String, String)> {
        let mut params = Vec::new();
        if let Some(min_id) = self.min_id {
            params.push(("minId".to_string(), min_id.to_string()));
        }
        if let Some(max_id) = self.max_id {
            params.push(("maxId".to_string(), max_id.to_string()));
        }
        if let Some(count) = self.count {
            params.push(("count".to_string(), count.to_string()));
        }
        if let Some(order) = self.order {
            params.push(("order".to_string(), order.to_string()));
        }
        params
    }
}
